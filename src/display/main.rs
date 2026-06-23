mod network;
mod ui;

use iced::{
    Application, Command, Element, Settings, Subscription, Theme,
};
use std::path::PathBuf;

#[path = "../shared.rs"]
mod shared;

use shared::SystemData;

const CONFIG_FILE: &str = "gpu_monitor_display.json";

fn main() -> iced::Result {
    tracing_subscriber::fmt()
        .with_target(false)
        .init();

    MonitorApp::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(1400.0, 900.0),
            min_size: Some(iced::Size::new(1000.0, 700.0)),
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct AppConfig {
    last_ip: String,
}

impl AppConfig {
    fn load() -> Self {
        let path = Self::config_path();
        if let Ok(data) = std::fs::read_to_string(&path) {
            if let Ok(cfg) = serde_json::from_str::<AppConfig>(&data) {
                return cfg;
            }
        }
        Self { last_ip: String::new() }
    }

    fn save(&self) {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(&path, data);
        }
    }

    fn config_path() -> PathBuf {
        let base = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."));
        base.join(CONFIG_FILE)
    }
}

#[derive(Debug, Clone)]
pub enum AppState {
    Connecting {
        ip_input: String,
        error: Option<String>,
    },
    WaitingForData {
        collector_addr: String,
    },
    Connected {
        collector_addr: String,
        data: SystemData,
        history: Vec<SystemData>,
        last_update: std::time::Instant,
    },
    Disconnected {
        collector_addr: String,
        error: String,
    },
}

#[derive(Debug, Clone)]
pub struct MonitorApp {
    state: AppState,
    style: ui::AppStyle,
    style_index: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    IpChanged(String),
    Connect(String),
    Disconnect,
    NetworkEvent(network::Event),
    Tick,
    ToggleStyle,
}

impl Application for MonitorApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let styles = ui::create_styles();
        let config = AppConfig::load();

        let initial_state = if config.last_ip.is_empty() {
            AppState::Connecting {
                ip_input: String::new(),
                error: None,
            }
        } else {
            AppState::WaitingForData {
                collector_addr: config.last_ip,
            }
        };

        (
            Self {
                state: initial_state,
                style: styles[0].clone(),
                style_index: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        let status = match &self.state {
            AppState::Connecting { .. } => "Connecting",
            AppState::WaitingForData { .. } => "Waiting...",
            AppState::Connected { .. } => "Live",
            AppState::Disconnected { .. } => "Disconnected",
        };
        format!("GPU Monitor [{}]", status)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IpChanged(ip) => {
                if let AppState::Connecting { ip_input, .. } = &mut self.state {
                    *ip_input = ip;
                }
                Command::none()
            }
            Message::Connect(addr) => {
                let mut config = AppConfig::load();
                config.last_ip = addr.clone();
                config.save();
                self.state = AppState::WaitingForData {
                    collector_addr: addr,
                };
                Command::none()
            }
            Message::Disconnect => {
                if let Some(addr) = self.collector_addr() {
                    self.state = AppState::Disconnected {
                        collector_addr: addr,
                        error: "Disconnected by user".to_string(),
                    };
                } else {
                    self.state = AppState::Connecting {
                        ip_input: String::new(),
                        error: None,
                    };
                }
                Command::none()
            }
            Message::NetworkEvent(event) => {
                match event {
                    network::Event::Connected => {
                        if let AppState::WaitingForData { collector_addr } = &self.state {
                            let addr = collector_addr.clone();
                            self.state = AppState::Connected {
                                collector_addr: addr,
                                data: SystemData {
                                    cpu_usage: 0.0,
                                    cpu_frequency: 0.0,
                                    cpu_temperature: 0.0,
                                    memory_usage: 0.0,
                                    memory_used: 0,
                                    memory_total: 0,
                                    gpu: None,
                                },
                                history: Vec::new(),
                                last_update: std::time::Instant::now(),
                            };
                        }
                    }
                    network::Event::Data(data) => {
                        match &mut self.state {
                            AppState::Connected {
                                data: current,
                                history,
                                last_update,
                                ..
                            } => {
                                history.push(data.clone());
                                if history.len() > 120 {
                                    history.remove(0);
                                }
                                *current = data;
                                *last_update = std::time::Instant::now();
                            }
                            AppState::WaitingForData { collector_addr } => {
                                let addr = collector_addr.clone();
                                self.state = AppState::Connected {
                                    collector_addr: addr,
                                    data: data.clone(),
                                    history: vec![data],
                                    last_update: std::time::Instant::now(),
                                };
                            }
                            _ => {}
                        }
                    }
                    network::Event::ConnectionFailed(err) => {
                        if let Some(addr) = self.collector_addr() {
                            self.state = AppState::Disconnected {
                                collector_addr: addr,
                                error: err,
                            };
                        }
                    }
                    network::Event::Error(err) => {
                        if let Some(addr) = self.collector_addr() {
                            self.state = AppState::Disconnected {
                                collector_addr: addr,
                                error: err,
                            };
                        }
                    }
                }
                Command::none()
            }
            Message::Tick => Command::none(),
            Message::ToggleStyle => {
                let styles = ui::create_styles();
                self.style_index = (self.style_index + 1) % styles.len();
                self.style = styles[self.style_index].clone();
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        match &self.state {
            AppState::Connecting { ip_input, error } => {
                ui::connection_view(ip_input, error.as_deref(), &self.style)
            }
            AppState::WaitingForData { collector_addr } => {
                ui::waiting_view(collector_addr, &self.style)
            }
            AppState::Connected {
                data,
                history,
                collector_addr,
                ..
            } => ui::monitor_view(data, history, collector_addr, &self.style),
            AppState::Disconnected { error, collector_addr } => {
                ui::error_view(error, collector_addr, &self.style)
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        match &self.state {
            AppState::WaitingForData { collector_addr }
            | AppState::Connected {
                collector_addr, ..
            } => {
                let addr = collector_addr.clone();
                network::subscribe(addr).map(Message::NetworkEvent)
            }
            AppState::Connecting { .. } | AppState::Disconnected { .. } => Subscription::none(),
        }
    }
}

impl MonitorApp {
    fn collector_addr(&self) -> Option<String> {
        match &self.state {
            AppState::WaitingForData { collector_addr }
            | AppState::Connected { collector_addr, .. }
            | AppState::Disconnected { collector_addr, .. } => Some(collector_addr.clone()),
            AppState::Connecting { .. } => None,
        }
    }
}
