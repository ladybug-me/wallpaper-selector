use super::copy_extent;
use crate::rendering::scene::pipeline::textures::near_mip_levels;

#[test]
fn rgba_growth_mip_extent() {
    assert_eq!(copy_extent(360, 2, wgpu::TextureFormat::Rgba8UnormSrgb), 90);
}

#[test]
fn bc_growth_keeps_block_alignment() {
    assert_eq!(copy_extent(360, 2, wgpu::TextureFormat::Bc7RgbaUnormSrgb), 92);
}

#[test]
fn rgba_atlas_single_level() {
    assert_eq!(near_mip_levels(wgpu::TextureFormat::Rgba8UnormSrgb), 1);
    assert!(near_mip_levels(wgpu::TextureFormat::Bc7RgbaUnormSrgb) > 1);
}
