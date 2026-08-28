use super::super::atlas;
use super::model::ScenePipeline;
use super::textures::{make_array_texture, make_near_texture, near_mip_levels};

impl ScenePipeline {
    pub(super) fn recreate_far(&mut self, device: &wgpu::Device, layers: u32) {
        self.far_tex =
            make_array_texture(device, layers, "skwd far atlas", self.far_format, self.atlas_size);
        self.far_layers = layers;
        self.bind_group = Self::build_bind_group(
            device,
            &self.bind_layout,
            &self.globals,
            &self.near_tex,
            &self.far_tex,
            &self.preview_tex,
            &self.sampler,
        );
    }

    pub(super) fn recreate_near(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layers: u32,
    ) {
        let new_texture = make_near_texture(device, self.near_format, layers);
        let layers_to_keep = self.near_layers.min(layers);
        if layers_to_keep > 0 {
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("skwd near grow"),
            });
            for mip in 0..near_mip_levels(self.near_format) {
                let width = copy_extent(atlas::NEAR_W, mip, self.near_format);
                let height = copy_extent(atlas::NEAR_H, mip, self.near_format);
                encoder.copy_texture_to_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &self.near_tex,
                        mip_level: mip,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    wgpu::TexelCopyTextureInfo {
                        texture: &new_texture,
                        mip_level: mip,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    wgpu::Extent3d { width, height, depth_or_array_layers: layers_to_keep },
                );
            }
            queue.submit(Some(encoder.finish()));
        }
        self.near_tex = new_texture;
        self.near_layers = layers;
        self.bind_group = Self::build_bind_group(
            device,
            &self.bind_layout,
            &self.globals,
            &self.near_tex,
            &self.far_tex,
            &self.preview_tex,
            &self.sampler,
        );
        self.sandy_bind = None;
    }
}

fn copy_extent(base: u32, mip: u32, format: wgpu::TextureFormat) -> u32 {
    let extent = (base >> mip).max(1);
    if format.is_compressed() { (extent + 3) & !3 } else { extent }
}

#[cfg(test)]
#[path = "atlas_resources_tests.rs"]
mod tests;
