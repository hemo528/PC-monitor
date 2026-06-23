use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuData {
    pub name: String,
    pub utilization: f32,
    pub temperature: f32,
    pub memory_utilization: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub power_usage: f32,
    pub clock_speed: u32,
    pub fan_speed: f32,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemData {
    pub cpu_usage: f32,
    pub cpu_frequency: f32,
    pub cpu_temperature: f32,
    pub memory_usage: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub gpu: Option<GpuData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MonitorMessage {
    DataUpdate(SystemData),
    Heartbeat,
    Error(String),
}

pub const BROADCAST_PORT: u16 = 9876;
pub const UPDATE_INTERVAL: Duration = Duration::from_millis(500);

impl SystemData {
    pub fn now_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
}
