use super::bindings::vsfs_pipeline;
use super::model::{SandyTarget, SandyUniformRaw, ScenePipeline};

pub(super) fn make_sandy_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("skwd sandy layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2Array,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 4,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
        ],
    })
}

pub(super) fn make_sandy_uniform(device: &wgpu::Device) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("skwd sandy uniform"),
        size: std::mem::size_of::<SandyUniformRaw>() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

pub(super) fn make_sandy_blit_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("skwd sandy blit layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    })
}

impl ScenePipeline {
    pub(super) fn release_sandy_target(&mut self) {
        self.sandy_blit_bind = None;
        self.sandy_target = None;
    }

    pub(super) fn ensure_sandy(&mut self, device: &wgpu::Device) {
        if self.sandy_pipeline.is_none() {
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("skwd sandy"),
                source: wgpu::ShaderSource::Wgsl(
                    include_str!("../../../../shaders/sandy.wgsl").into(),
                ),
            });
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("skwd sandy pipeline layout"),
                bind_group_layouts: &[&self.sandy_layout],
                push_constant_ranges: &[],
            });
            self.sandy_pipeline =
                Some(vsfs_pipeline(device, "skwd sandy pipeline", &layout, &shader, self.format));
        }
        if self.sandy_bind.is_none() {
            let near_view = self.near_tex.create_view(&wgpu::TextureViewDescriptor {
                dimension: Some(wgpu::TextureViewDimension::D2Array),
                ..Default::default()
            });
            let preview_view =
                self.preview_tex.create_view(&wgpu::TextureViewDescriptor::default());
            let preview_out_view =
                self.preview_out_tex.create_view(&wgpu::TextureViewDescriptor::default());
            self.sandy_bind = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("skwd sandy bind"),
                layout: &self.sandy_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.sandy_uniform.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&near_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Sampler(&self.sampler),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::TextureView(&preview_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: wgpu::BindingResource::TextureView(&preview_out_view),
                    },
                ],
            }));
        }
    }

    pub(super) fn ensure_sandy_blit(&mut self, device: &wgpu::Device) {
        if self.sandy_blit_pipeline.is_some() {
            return;
        }
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("skwd sandy blit"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("../../../../shaders/sandy_blit.wgsl").into(),
            ),
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("skwd sandy blit pipeline layout"),
            bind_group_layouts: &[&self.sandy_blit_layout],
            push_constant_ranges: &[],
        });
        self.sandy_blit_pipeline =
            Some(vsfs_pipeline(device, "skwd sandy blit pipeline", &layout, &shader, self.format));
    }

    pub(super) fn ensure_sandy_target(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        if let Some(target) = &self.sandy_target
            && target.width == width
            && target.height == height
        {
            return;
        }
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("skwd sandy target"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        self.sandy_blit_bind = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("skwd sandy blit bind"),
            layout: &self.sandy_blit_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        }));
        self.sandy_target = Some(SandyTarget { view, width, height });
    }
}
