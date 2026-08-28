use std::{marker::PhantomData, sync::Arc};

use iced::Rectangle;
use iced::widget::shader::{self, Action, Pipeline, Primitive, Viewport};
use iced::{Element, Event, Length, mouse};

use crate::contracts::rendering::{PreviewRenderer, PreviewRequest};

pub struct GpuPreviewRenderer;

impl<Message> PreviewRenderer<Message> for GpuPreviewRenderer {
    type Output = Element<'static, Message>;

    fn view(&self, request: PreviewRequest) -> Element<'static, Message>
    where
        Message: 'static,
    {
        iced::widget::shader(EffectProgram::<Message> { request, message: PhantomData })
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    bounds: [f32; 4],
    tex_size: [f32; 2],
    effect: u32,
    out_srgb: u32,
    params: [f32; 4],
    color_a: [f32; 4],
    color_b: [f32; 4],
}

struct EffectProgram<Message> {
    request: PreviewRequest,
    message: PhantomData<fn() -> Message>,
}

impl<Message> shader::Program<Message> for EffectProgram<Message> {
    type State = ();
    type Primitive = EffectPrimitive;

    fn update(
        &self,
        _state: &mut (),
        _event: &Event,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Option<Action<Message>> {
        None
    }

    fn draw(&self, _state: &(), _cursor: mouse::Cursor, _bounds: Rectangle) -> EffectPrimitive {
        EffectPrimitive {
            rgba: self.request.rgba.clone(),
            width: self.request.width,
            height: self.request.height,
            version: self.request.version,
            effect: self.request.shader.effect,
            params: self.request.shader.params,
            color_a: self.request.shader.color_a,
            color_b: self.request.shader.color_b,
        }
    }
}

#[derive(Debug)]
pub struct EffectPrimitive {
    rgba: Arc<Vec<u8>>,
    width: u32,
    height: u32,
    version: u64,
    effect: u32,
    params: [f32; 4],
    color_a: [f32; 4],
    color_b: [f32; 4],
}

impl Primitive for EffectPrimitive {
    type Pipeline = EffectPipeline;

    fn prepare(
        &self,
        pipeline: &mut EffectPipeline,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bounds: &Rectangle,
        viewport: &Viewport,
    ) {
        let stale = pipeline.tex.is_none()
            || pipeline.tex_version != self.version
            || pipeline.tex_w != self.width
            || pipeline.tex_h != self.height;
        if stale && self.width > 0 && self.height > 0 && !self.rgba.is_empty() {
            let tex = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("skwd effect src"),
                size: wgpu::Extent3d {
                    width: self.width,
                    height: self.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &tex,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &self.rgba,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(self.width * 4),
                    rows_per_image: Some(self.height),
                },
                wgpu::Extent3d { width: self.width, height: self.height, depth_or_array_layers: 1 },
            );
            let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
            pipeline.bind = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("skwd effect bind"),
                layout: &pipeline.layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: pipeline.uniform.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Sampler(&pipeline.sampler),
                    },
                ],
            }));
            pipeline.tex = Some(tex);
            pipeline.tex_version = self.version;
            pipeline.tex_w = self.width;
            pipeline.tex_h = self.height;
        }

        let sf = viewport.scale_factor();
        let uniforms = Uniforms {
            bounds: [
                bounds.x * sf,
                bounds.y * sf,
                (bounds.width * sf).max(1.0),
                (bounds.height * sf).max(1.0),
            ],
            tex_size: [self.width.max(1) as f32, self.height.max(1) as f32],
            effect: self.effect,
            out_srgb: u32::from(pipeline.out_srgb),
            params: self.params,
            color_a: self.color_a,
            color_b: self.color_b,
        };
        queue.write_buffer(&pipeline.uniform, 0, bytemuck::bytes_of(&uniforms));
    }

    fn draw(&self, _pipeline: &EffectPipeline, _render_pass: &mut wgpu::RenderPass<'_>) -> bool {
        false
    }

    fn render(
        &self,
        pipeline: &EffectPipeline,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        clip_bounds: &Rectangle<u32>,
    ) {
        let Some(bind) = &pipeline.bind else {
            return;
        };
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("skwd effect preview"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        pass.set_scissor_rect(clip_bounds.x, clip_bounds.y, clip_bounds.width, clip_bounds.height);
        pass.set_pipeline(&pipeline.pipeline);
        pass.set_bind_group(0, bind, &[]);
        pass.draw(0..3, 0..1);
    }
}

pub struct EffectPipeline {
    pipeline: wgpu::RenderPipeline,
    uniform: wgpu::Buffer,
    sampler: wgpu::Sampler,
    layout: wgpu::BindGroupLayout,
    out_srgb: bool,
    tex: Option<wgpu::Texture>,
    tex_version: u64,
    tex_w: u32,
    tex_h: u32,
    bind: Option<wgpu::BindGroup>,
}

impl std::fmt::Debug for EffectPipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EffectPipeline").finish()
    }
}

impl Pipeline for EffectPipeline {
    fn new(device: &wgpu::Device, _queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("skwd effect preview"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!("../../../shaders/effect_preview.wgsl").into(),
            ),
        });
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("skwd effect uniform"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("skwd effect sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("skwd effect layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("skwd effect pipeline layout"),
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("skwd effect pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        Self {
            pipeline,
            uniform,
            sampler,
            layout,
            out_srgb: format.is_srgb(),
            tex: None,
            tex_version: 0,
            tex_w: 0,
            tex_h: 0,
            bind: None,
        }
    }
}

#[cfg(test)]
#[path = "pipeline_tests.rs"]
mod tests;
