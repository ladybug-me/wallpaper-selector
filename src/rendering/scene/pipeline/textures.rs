use super::super::atlas;
use crate::contracts::rendering::InstanceRaw;

pub(super) fn physical_array_layers(logical_layers: u32) -> u32 {
    let layers = logical_layers.max(2);
    if layers.is_multiple_of(6) { layers + 1 } else { layers }
}

pub(super) fn make_array_texture(
    device: &wgpu::Device,
    layers: u32,
    label: &str,
    format: wgpu::TextureFormat,
    size: u32,
) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: physical_array_layers(layers),
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    })
}

pub(super) fn make_near_texture(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    layers: u32,
) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("skwd near atlas"),
        size: wgpu::Extent3d {
            width: atlas::NEAR_W,
            height: atlas::NEAR_H,
            depth_or_array_layers: physical_array_layers(layers),
        },
        mip_level_count: near_mip_levels(format),
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    })
}

pub(super) fn make_preview_texture(
    device: &wgpu::Device,
    label: &str,
    usage: wgpu::TextureUsages,
) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d {
            width: atlas::NEAR_W,
            height: atlas::NEAR_H,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage,
        view_formats: &[],
    })
}

pub(super) fn near_mip_levels(format: wgpu::TextureFormat) -> u32 {
    if format.is_compressed() { atlas::NEAR_MIPS } else { 1 }
}

pub(super) fn upload_instances(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    buffer: &mut wgpu::Buffer,
    capacity: &mut u64,
    data: &[InstanceRaw],
) -> u32 {
    let bytes: &[u8] = bytemuck::cast_slice(data);
    if bytes.is_empty() {
        return 0;
    }
    if bytes.len() as u64 > *capacity {
        *buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("skwd instances"),
            size: (bytes.len() as u64).next_power_of_two(),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        *capacity = buffer.size();
    }
    queue.write_buffer(buffer, 0, bytes);
    data.len() as u32
}

pub(super) fn upload_near_container(
    queue: &wgpu::Queue,
    texture: &wgpu::Texture,
    layer: u32,
    data: &[u8],
) {
    if data.len() < 12 || &data[0..4] != b"SKB1" {
        return;
    }
    if data.len() < 12 + (data[5] as usize) * 8 {
        return;
    }
    let levels = (data[5] as u32).min(atlas::NEAR_MIPS) as usize;
    let index_offset = 12;
    let mut data_offset = index_offset + (data[5] as usize) * 8;
    for mip in 0..levels {
        let index = index_offset + mip * 8;
        let width = u16::from_le_bytes([data[index], data[index + 1]]) as u32;
        let height = u16::from_le_bytes([data[index + 2], data[index + 3]]) as u32;
        let length = u32::from_le_bytes([
            data[index + 4],
            data[index + 5],
            data[index + 6],
            data[index + 7],
        ]) as usize;
        if data_offset + length > data.len() {
            return;
        }
        let aligned_width = (width + 3) & !3;
        let aligned_height = (height + 3) & !3;
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture,
                mip_level: mip as u32,
                origin: wgpu::Origin3d { x: 0, y: 0, z: layer },
                aspect: wgpu::TextureAspect::All,
            },
            &data[data_offset..data_offset + length],
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some((aligned_width / 4) * 16),
                rows_per_image: Some(aligned_height / 4),
            },
            wgpu::Extent3d {
                width: aligned_width,
                height: aligned_height,
                depth_or_array_layers: 1,
            },
        );
        data_offset += length;
    }
}

pub(super) fn upload_near_rgba(
    queue: &wgpu::Queue,
    texture: &wgpu::Texture,
    layer: u32,
    width: u32,
    height: u32,
    data: &[u8],
) {
    if data.len() != (width as usize) * (height as usize) * 4 {
        return;
    }
    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture,
            mip_level: 0,
            origin: wgpu::Origin3d { x: 0, y: 0, z: layer },
            aspect: wgpu::TextureAspect::All,
        },
        data,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(width * 4),
            rows_per_image: Some(height),
        },
        wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
    );
}
