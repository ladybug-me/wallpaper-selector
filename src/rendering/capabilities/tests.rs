#![cfg(test)]
use crate::contracts::capabilities::GraphicsTier;

use super::gpu::*;

#[test]
fn lod_auto_gates_tier() {
    assert_eq!(effective_lod(2.0, true, GraphicsTier::Discrete), 1.0);
    assert_eq!(effective_lod(2.0, true, GraphicsTier::Integrated), 2.0);
    assert_eq!(effective_lod(2.0, true, GraphicsTier::Other), 2.0);
    assert_eq!(effective_lod(2.0, false, GraphicsTier::Discrete), 2.0);
    assert_eq!(effective_lod(1.0, true, GraphicsTier::Integrated), 1.0);
}

#[test]
fn device_type_tiers() {
    assert_eq!(classify(wgpu::DeviceType::DiscreteGpu), GraphicsTier::Discrete);
    assert_eq!(classify(wgpu::DeviceType::IntegratedGpu), GraphicsTier::Integrated);
    assert_eq!(classify(wgpu::DeviceType::VirtualGpu), GraphicsTier::Other);
}
