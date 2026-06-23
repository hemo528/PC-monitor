use anyhow::Result;
use nvml_wrapper::Nvml;
use tracing::{info, warn};

use crate::shared::GpuData;

pub struct GpuMonitor {
    nvml: Nvml,
    device_index: u32,
}

impl GpuMonitor {
    pub fn new() -> Option<Self> {
        match Nvml::init() {
            Ok(nvml) => match nvml.device_count() {
                Ok(count) if count > 0 => {
                    info!("Found {} GPU(s)", count);
                    Some(Self {
                        nvml,
                        device_index: 0,
                    })
                }
                Ok(_) => {
                    warn!("No GPU devices found");
                    None
                }
                Err(e) => {
                    warn!("Failed to get device count: {}", e);
                    None
                }
            },
            Err(e) => {
                warn!("Failed to initialize NVML: {}", e);
                None
            }
        }
    }

    pub fn name(&self) -> String {
        self.nvml
            .device_by_index(self.device_index)
            .and_then(|device| device.name())
            .unwrap_or_else(|_| "Unknown GPU".to_string())
    }

    pub fn get_data(&self) -> Result<GpuData> {
        let device = self.nvml.device_by_index(self.device_index)?;

        let utilization = device.utilization_rates()?;
        let temperature = device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)?;
        let memory_info = device.memory_info()?;
        let power = device.power_usage()?;
        let clock = device.clock_info(nvml_wrapper::enum_wrappers::device::Clock::Graphics)?;
        let fan_speed = device.fan_speed(0).unwrap_or(0);

        Ok(GpuData {
            name: self.name(),
            utilization: utilization.gpu as f32,
            temperature: temperature as f32,
            memory_utilization: if memory_info.total > 0 {
                (memory_info.used as f32 / memory_info.total as f32) * 100.0
            } else {
                0.0
            },
            memory_used: memory_info.used / (1024 * 1024),
            memory_total: memory_info.total / (1024 * 1024),
            power_usage: power as f32 / 1000.0,
            clock_speed: clock,
            fan_speed: fan_speed as f32,
            timestamp: crate::shared::SystemData::now_timestamp(),
        })
    }
}

