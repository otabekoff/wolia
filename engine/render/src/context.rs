//! Render context and GPU resources.

use crate::{Error, Result};

/// GPU render context.
pub struct RenderContext {
    /// The wgpu instance.
    pub instance: wgpu::Instance,
    /// The GPU adapter.
    pub adapter: wgpu::Adapter,
    /// The GPU device.
    pub device: wgpu::Device,
    /// The command queue.
    pub queue: wgpu::Queue,
}

impl RenderContext {
    /// Create a new render context.
    pub async fn new() -> Result<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| Error::Gpu("No suitable GPU adapter found".to_string()))?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Wolia Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .map_err(|e| Error::Gpu(e.to_string()))?;

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
        })
    }

    /// Get the device.
    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    /// Get the queue.
    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }
}
