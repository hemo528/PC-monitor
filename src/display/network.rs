use iced::futures::SinkExt;
use iced::subscription::{self, Subscription};
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use tokio::sync::mpsc;
use tokio::task;

use crate::shared::{MonitorMessage, SystemData};

const CONNECT_TIMEOUT_MS: u64 = 1500;
const READ_TIMEOUT_SECS: u64 = 15;
const RECONNECT_DELAY_MS: u64 = 800;

#[derive(Debug, Clone)]
pub enum Event {
    Connected,
    Data(SystemData),
    ConnectionFailed(String),
    Error(String),
}

pub fn subscribe(collector_addr: String) -> Subscription<Event> {
    subscription::channel(
        std::any::TypeId::of::<NetworkState>(),
        100,
        |mut sender| async move {
            loop {
                let addr = collector_addr.clone();

                let (tx, mut rx) = mpsc::channel(100);

                let handle = task::spawn_blocking(move || {
                    connect_and_listen(&addr, tx);
                });

                while let Some(event) = rx.recv().await {
                    let _ = sender.send(event).await;
                }

                if let Err(err) = handle.await {
                    tracing::error!("TCP task failed: {}", err);
                }

                let _ = sender
                    .send(Event::ConnectionFailed(
                        "Disconnected, reconnecting...".to_string(),
                    ))
                    .await;

                tokio::time::sleep(std::time::Duration::from_millis(RECONNECT_DELAY_MS)).await;
            }
        },
    )
}

#[derive(Debug)]
struct NetworkState;

fn connect_and_listen(collector_addr: &str, tx: mpsc::Sender<Event>) {
    let addr = if collector_addr.contains(':') {
        collector_addr.to_string()
    } else {
        format!("{}:9876", collector_addr)
    };

    tracing::info!("Connecting to {}", addr);

    let socket_addr: std::net::SocketAddr = match addr.parse() {
        Ok(a) => a,
        Err(_) => {
            let _ = tx.blocking_send(Event::ConnectionFailed(format!("Invalid address: {}", addr)));
            return;
        }
    };

    let stream = match TcpStream::connect_timeout(
        &socket_addr,
        std::time::Duration::from_millis(CONNECT_TIMEOUT_MS),
    ) {
        Ok(s) => s,
        Err(e) => {
            let _ = tx.blocking_send(Event::ConnectionFailed(format!(
                "Cannot connect to {}: {}",
                addr, e
            )));
            return;
        }
    };

    let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(READ_TIMEOUT_SECS)));
    let _ = stream.set_nodelay(true);

    tracing::info!("Connected to {}", addr);
    let _ = tx.blocking_send(Event::Connected);

    let reader = BufReader::new(stream);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                match serde_json::from_str::<MonitorMessage>(trimmed) {
                    Ok(MonitorMessage::DataUpdate(data)) => {
                        if tx.blocking_send(Event::Data(data)).is_err() {
                            break;
                        }
                    }
                    Ok(MonitorMessage::Error(err)) => {
                        if tx.blocking_send(Event::Error(err)).is_err() {
                            break;
                        }
                    }
                    Ok(MonitorMessage::Heartbeat) => {}
                    Err(_) => {}
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                let _ = tx.blocking_send(Event::ConnectionFailed(
                    "No data received".to_string(),
                ));
                break;
            }
            Err(e) => {
                let _ = tx.blocking_send(Event::Error(format!("Error: {}", e)));
                break;
            }
        }
    }

    tracing::info!("Disconnected from {}", addr);
}
