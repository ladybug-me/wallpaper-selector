use crate::contracts::capabilities::{GraphicsCard, GraphicsProbe, GraphicsTier};

#[derive(Clone, Copy, Debug, Default)]
pub struct WgpuGraphicsProbe;

impl GraphicsProbe for WgpuGraphicsProbe {
    fn probe(&self) -> GraphicsCard {
        let started = std::time::Instant::now();
        let backends = wgpu::Backends::VULKAN;
        let instance =
            wgpu::Instance::new(&wgpu::InstanceDescriptor { backends, ..Default::default() });
        let preference = wgpu::PowerPreference::from_env().unwrap_or(wgpu::PowerPreference::None);
        let selected = iced::futures::executor::block_on(instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: preference,
                compatible_surface: None,
                force_fallback_adapter: false,
            },
        ))
        .ok();
        log::info!(
            "gpu tier probe: selected {preference:?} adapter in {} ms",
            started.elapsed().as_millis()
        );
        match selected {
            Some(adapter) => {
                let info = adapter.get_info();
                GraphicsCard { name: info.name, tier: classify(info.device_type) }
            }
            None => GraphicsCard { name: String::from("unknown"), tier: GraphicsTier::Other },
        }
    }
}

pub(super) fn classify(device_type: wgpu::DeviceType) -> GraphicsTier {
    match device_type {
        wgpu::DeviceType::DiscreteGpu => GraphicsTier::Discrete,
        wgpu::DeviceType::IntegratedGpu => GraphicsTier::Integrated,
        _ => GraphicsTier::Other,
    }
}

pub fn effective_lod(lod: f32, auto: bool, tier: GraphicsTier) -> f32 {
    if auto && tier == GraphicsTier::Discrete { 1.0 } else { lod }
}
