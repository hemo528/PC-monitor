mod gpu;

use std::io::Write;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use anyhow::Result;
use sysinfo::{CpuRefreshKind, RefreshKind, System};
use tracing::{info, warn};

#[path = "../shared.rs"]
mod shared;

use shared::{MonitorMessage, SystemData, UPDATE_INTERVAL};

const DEFAULT_PORT: u16 = 9876;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .init();

    let args: Vec<String> = std::env::args().collect();
    let port: u16 = args
        .get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_PORT);

    info!("GPU Monitor Collector starting...");

    let mut system = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(sysinfo::MemoryRefreshKind::everything()),
    );
    system.refresh_all();

    let gpu_monitor = gpu::GpuMonitor::new();
    match &gpu_monitor {
        Some(monitor) => info!("GPU detected: {}", monitor.name()),
        None => warn!("No NVIDIA GPU detected or NVML not available"),
    }

    // Shared CPU sensor data, updated by background thread
    let cpu_sensors: Arc<Mutex<CpuSensors>> = Arc::new(Mutex::new(CpuSensors::default()));
    let sensors_bg = cpu_sensors.clone();
    std::thread::spawn(move || {
        loop {
            let data = poll_cpu_sensors();
            if let Ok(mut s) = sensors_bg.lock() {
                *s = data;
            }
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    });

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;
    listener.set_nonblocking(true)?;

    info!("Collector listening on TCP port {}", port);

    let clients: Arc<Mutex<Vec<std::net::TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    let clients_accept = clients.clone();
    std::thread::spawn(move || {
        loop {
            match listener.accept() {
                Ok((stream, addr)) => {
                    info!("Display connected from {}", addr);
                    let _ = stream.set_nodelay(true);
                    let _ = stream.set_nonblocking(true);
                    if let Ok(mut guard) = clients_accept.lock() {
                        guard.push(stream);
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
                Err(e) => {
                    warn!("Accept error: {}", e);
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
            }
        }
    });

    let mut last_update = Instant::now();
    let mut frame_count: u64 = 0;

    info!("Collector started. Waiting for display connections...");

    loop {
        if last_update.elapsed() >= UPDATE_INTERVAL {
            system.refresh_all();

            let cpu_usage = system.global_cpu_info().cpu_usage();

            let sensors = cpu_sensors.lock().map(|s| s.clone()).unwrap_or_default();

            let memory_total = system.total_memory() / (1024 * 1024);
            let memory_used = system.used_memory() / (1024 * 1024);
            let memory_usage = if memory_total > 0 {
                memory_used as f32 / memory_total as f32 * 100.0
            } else {
                0.0
            };

            let gpu_data = gpu_monitor.as_ref().and_then(|monitor| match monitor.get_data() {
                Ok(data) => Some(data),
                Err(err) => {
                    warn!("Failed to get GPU data: {}", err);
                    None
                }
            });

            let payload = MonitorMessage::DataUpdate(SystemData {
                cpu_usage,
                cpu_frequency: sensors.frequency,
                cpu_temperature: sensors.temperature,
                memory_usage,
                memory_used,
                memory_total,
                gpu: gpu_data,
            });

            if let Ok(bytes) = serde_json::to_vec(&payload) {
                let mut data = bytes;
                data.push(b'\n');

                if let Ok(mut guard) = clients.lock() {
                    guard.retain_mut(|stream| {
                        match stream.write_all(&data) {
                            Ok(()) => true,
                            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => true,
                            Err(_) => false,
                        }
                    });
                }
            }

            frame_count += 1;
            if frame_count % 100 == 0 {
                let client_count = clients.lock().map(|g| g.len()).unwrap_or(0);
                info!(
                    "Sent {} frames, {} display(s), cpu={:.0} MHz, temp={:.1}°C",
                    frame_count, client_count, sensors.frequency, sensors.temperature
                );
            }

            last_update = Instant::now();
        }

        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

#[derive(Debug, Clone, Default)]
struct CpuSensors {
    frequency: f32,
    temperature: f32,
}

/// Poll CPU frequency and temperature from WMI Performance Counters.
/// This matches what Windows Task Manager shows.
fn poll_cpu_sensors() -> CpuSensors {
    let mut sensors = CpuSensors::default();

    // Get base frequency and percent performance to calculate real-time frequency
    let Ok(output) = std::process::Command::new("powershell")
        .args([
            "-NoProfile", "-Command",
            r#"
            $p = Get-CimInstance Win32_PerfFormattedData_Counters_ProcessorInformation -EA SilentlyContinue |
                 Where-Object { $_.Name -eq '_Total' } |
                 Select-Object -First 1 ProcessorFrequency, PercentProcessorPerformance
            $t = Get-CimInstance Win32_PerfFormattedData_Counters_ThermalZoneInformation -EA SilentlyContinue |
                 Select-Object -First 1 Temperature
            if ($p -and $t) {
                "$($p.ProcessorFrequency)|$($p.PercentProcessorPerformance)|$($t.Temperature)"
            } elseif ($p) {
                "$($p.ProcessorFrequency)|$($p.PercentProcessorPerformance)|0"
            }
            "#,
        ])
        .output()
    else {
        return sensors;
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let trimmed = stdout.trim();
    let parts: Vec<&str> = trimmed.split('|').collect();

    if parts.len() >= 2 {
        if let Ok(base_freq) = parts[0].parse::<f32>() {
            if let Ok(pct_perf) = parts[1].parse::<f32>() {
                // Real-time frequency = base_freq * percent_performance / 100
                // This matches Task Manager's "Speed" column
                sensors.frequency = base_freq * pct_perf / 100.0;
            }
        }
    }

    if parts.len() >= 3 {
        if let Ok(raw_temp) = parts[2].parse::<f32>() {
            if raw_temp > 200.0 {
                // WMI returns temperature in Kelvin
                sensors.temperature = raw_temp - 273.15;
            } else if raw_temp > 0.0 {
                sensors.temperature = raw_temp;
            }
        }
    }

    sensors
}
