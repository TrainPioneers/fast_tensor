use bytemuck::cast_slice;
use wgpu::{
    Buffer,
    BufferDescriptor,
    BufferUsages,
    Device,
    util::{BufferInitDescriptor, DeviceExt}
};

pub async fn create_buffer(device: &Device, data: Vec<f32>) -> Buffer {
    device.create_buffer_init(&BufferInitDescriptor {
        label: Some("buffer"),
        contents: cast_slice(&*data),
        usage: BufferUsages::STORAGE,
    })
}

pub async fn create_result_buffer (device: &Device, size: u64) -> Buffer{
    device.create_buffer(&BufferDescriptor {
        label: Some("Result Buffer"),
        size,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    })
}

pub async fn create_staging_buffer(device: &Device, size: u64) -> Buffer {
    device.create_buffer(&BufferDescriptor {
        label: Some("Staging Buffer"),
        size,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}