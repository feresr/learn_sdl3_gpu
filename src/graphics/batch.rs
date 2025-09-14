use sdl3::gpu::{BufferBinding, ColorTargetInfo, Device};

use crate::graphics::Vertex;
use crate::graphics::material::Material;
use crate::graphics::mesh::Mesh;

pub struct Batch {
    device: Device,
    mesh: Mesh,
    material: Material,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    batches: Vec<DrawBatch>,
}

impl Batch {
    pub fn new(device: Device, default_material: Material) -> Self {
        Batch {
            device: device.clone(),
            mesh: Mesh::new(device),
            material: default_material,
            vertices: Default::default(),
            indices: Default::default(),
            batches: Default::default(),
        }
    }

    fn push_vertex(
        &mut self,
        position: [f32; 3],
        color: [u8; 4],
        texture_uv: [f32; 2],
        mult: u8,
        wash: u8,
        fill: u8,
    ) {
        self.vertices.push(Vertex {
            position,
            color,
            texture_uv,
            mult_wash_fill: [mult, wash, fill, 0],
        });
    }

    pub fn triangle(
        &mut self,
        position0: [f32; 3],
        position1: [f32; 3],
        position2: [f32; 3],
        color: [u8; 4],
    ) {
        let last_vertex_index = self.vertices.len() as u32;
        self.indices.extend([
            0 + last_vertex_index,
            1 + last_vertex_index,
            2 + last_vertex_index,
        ]);
        self.vertices.reserve(3);
        self.push_vertex(position0, color, [0f32, 0f32], 0, 0, 255);
        self.push_vertex(position1, color, [0f32, 0f32], 0, 0, 255);
        self.push_vertex(position2, color, [0f32, 0f32], 0, 0, 255);
        self.current_batch().elements += 1;
    }

    fn current_batch(&mut self) -> &mut DrawBatch {
        if self.batches.is_empty() {
            let value = DrawBatch {
                offset: 0,
                elements: 0,
            };
            self.batches.push(value);
        }
        return self.batches.last_mut().unwrap();
    }

    pub fn draw(&mut self, target: ColorTargetInfo) {
        self.mesh.set_data(&self.vertices);
        self.mesh.set_indices(&self.indices);

        // Copy pass
        self.mesh.upload();

        let cmd = self.device.acquire_command_buffer().unwrap();

        let render_pass = self
            .device
            .begin_render_pass(&cmd, &[target], None)
            .unwrap();

        render_pass.bind_graphics_pipeline(&self.material.pipeline);

        let buffer_binding = BufferBinding::new()
            .with_offset(0)
            .with_buffer(&self.mesh.vertex_buffer);
        let index_binding = BufferBinding::new()
            .with_offset(0)
            .with_buffer(&self.mesh.index_buffer);

        render_pass.bind_vertex_buffers(0, &[buffer_binding]);
        render_pass.bind_index_buffer(
            &index_binding,
            sdl3::sys::gpu::SDL_GPUIndexElementSize::_32BIT,
        );

        // TODO: update params
        for batch in &self.batches {
            render_pass.draw_indexed_primitives(
                (batch.elements * 3) as u32,
                1,
                0,
                batch.offset as i32,
                0,
            );
        }

        self.device.end_render_pass(render_pass);

        cmd.submit().unwrap();
    }
}

pub struct DrawBatch {
    offset: i64,
    elements: i64,
}
