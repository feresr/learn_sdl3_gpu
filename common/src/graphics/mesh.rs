use sdl3::gpu::{
    Buffer, BufferRegion, BufferUsageFlags, CopyPass, Device, TransferBuffer, TransferBufferLocation, TransferBufferUsage
};

use crate::graphics::{Vertex, MAX_INDICES, MAX_VERTICES};

pub struct Mesh {
    device: Device,
    pub vertex_buffer: Buffer,
    vertex_transfer_buffer: TransferBuffer,
    pub index_buffer: Buffer,
    index_transfer_buffer: TransferBuffer,
}

impl Mesh {
    pub fn new(device: Device) -> Self {
        let vertex_buffer = device
            .create_buffer()
            .with_usage(BufferUsageFlags::VERTEX)
            .with_size(size_of::<Vertex>() as u32 * MAX_VERTICES)
            .build()
            .unwrap();

        let vertex_transfer_buffer = device
            .create_transfer_buffer()
            .with_usage(TransferBufferUsage::UPLOAD)
            .with_size(size_of::<Vertex>() as u32 * MAX_VERTICES)
            .build()
            .unwrap();

        let index_buffer = device
            .create_buffer()
            .with_usage(BufferUsageFlags::INDEX)
            .with_size(size_of::<u32>() as u32 * MAX_INDICES)
            .build()
            .unwrap();

        let index_transfer_buffer = device
            .create_transfer_buffer()
            .with_usage(TransferBufferUsage::UPLOAD)
            .with_size(size_of::<u32>() as u32 * MAX_INDICES)
            .build()
            .unwrap();

        return Mesh {
            device,
            vertex_buffer,
            vertex_transfer_buffer,
            index_buffer,
            index_transfer_buffer,
        };
    }

    pub fn set_data(&mut self, vertices: &[Vertex]) {
        let mut map = self
            .vertex_transfer_buffer
            .map::<Vertex>(&self.device, true);
        let memory = map.mem_mut();
        // memory.copy_from_slice(&vertices);
        memory[..vertices.len()].copy_from_slice(&vertices);
        map.unmap();
    }

    pub fn set_indices(&mut self, indices: &[u32]) {
        let mut map = self.index_transfer_buffer.map::<u32>(&self.device, true);
        let memory = map.mem_mut();
        memory[..indices.len()].copy_from_slice(&indices);
        map.unmap();
    }

    pub fn upload(&mut self, copy_pass : &CopyPass) {

        // Upload vertices
        copy_pass.upload_to_gpu_buffer(
            TransferBufferLocation::new()
                .with_transfer_buffer(&self.vertex_transfer_buffer)
                .with_offset(0),
            BufferRegion::new()
                .with_buffer(&self.vertex_buffer)
                .with_size(self.vertex_buffer.len()),
            false,
        );
        // Upload indices
        copy_pass.upload_to_gpu_buffer(
            TransferBufferLocation::new()
                .with_transfer_buffer(&self.index_transfer_buffer)
                .with_offset(0),
            BufferRegion::new()
                .with_buffer(&self.index_buffer)
                .with_size(self.index_buffer.len()),
            false,
        );

    }
}
