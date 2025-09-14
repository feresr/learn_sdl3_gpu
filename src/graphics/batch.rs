use std::cell::RefCell;
use std::f32::consts::TAU;
use std::rc::Rc;

use sdl3::gpu::{BufferBinding, ColorTargetInfo, Device};
use sdl3::render;

use crate::graphics::Vertex;
use crate::graphics::material::Material;
use crate::graphics::mesh::Mesh;
use crate::graphics::texture::Texture;

pub struct Batch {
    device: Device,
    mesh: Mesh,
    material: Material,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    batches: Vec<DrawBatch>,
    texture: Rc<RefCell<Texture>>,
}

impl Batch {
    pub fn new(device: Device, default_material: Material) -> Self {
        let texture = Texture::from_path(
            device.clone(),
            "/Users/feresr/Workspace/learn_sdl3_gpu/src/atlas.png",
        );
        Batch {
            device: device.clone(),
            mesh: Mesh::new(device),
            material: default_material,
            vertices: Default::default(),
            indices: Default::default(),
            batches: Default::default(),
            texture: Rc::new(RefCell::new(texture)),
        }
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

    pub fn texture(&mut self, texture: Texture, position: glm::Vec2) {
        // TODO: Hardcoded + 1.0f32 should be texture width and height
        // (add projection matrix to the shader!)
        let position0 = [position.x, position.y, 0.0f32];
        let position1 = [position.x + 1.0f32, position.y, 0.0f32];
        let position2 = [position.x, position.y + 1.0f32, 0.0f32];
        let position3 = [
            position.x + 0.8f32, //texture.width as f32,
            position.y + 0.8f32, // texture.height as f32,
            0.0f32,
        ];

        self.push_quad(
            position0,
            position1,
            position2,
            position3,
            255,
            0,
            0,
            [255, 255, 255, 255],
        );
    }

    pub fn quad(
        &mut self,
        position0: [f32; 3],
        position1: [f32; 3],
        position2: [f32; 3],
        position3: [f32; 3],
        color: [u8; 4],
    ) {
        self.push_quad(position0, position1, position2, position3, 0, 0, 255, color);
    }

    pub fn circle(&mut self, center: [f32; 2], radius: f32, steps: u32, color: [u8; 4]) {
        let mut last = [center[0] + radius, center[1], 0.0];
        let center = [center[0], center[1], 0.0];
        let radians = (1 as f32 / steps as f32) * TAU;
        for i in 0..=steps {
            let next = [
                center[0] + f32::cos(radians * i as f32) * radius,
                center[1] + f32::sin(radians * i as f32) * radius,
                0.0,
            ];
            self.triangle(last, next, center, color);
            last = next;
        }
    }

    pub fn draw(&mut self, target: ColorTargetInfo) {
        self.mesh.set_data(&self.vertices);
        self.mesh.set_indices(&self.indices);

        // Copy pass

        let upload_cmd = self.device.acquire_command_buffer().unwrap();
        let copy_pass = self.device.begin_copy_pass(&upload_cmd).unwrap();

        self.texture.borrow_mut().upload(&copy_pass);
        self.mesh.upload(&copy_pass);

        self.device.end_copy_pass(copy_pass);
        upload_cmd.submit().unwrap();

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

        render_pass.bind_fragment_samplers(0, &[self.texture.borrow().bindings()]);

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

    fn push_quad(
        &mut self,
        position0: [f32; 3],
        position1: [f32; 3],
        position2: [f32; 3],
        position3: [f32; 3],
        mult: u8,
        wash: u8,
        fill: u8,
        color: [u8; 4],
    ) {
        let last_vertex_index = self.vertices.len() as u32;
        /*
         * 0 ---- 1
         * |      |
         * |      |
         * 2 ---- 3
         */
        self.indices.extend([
            0 + last_vertex_index,
            1 + last_vertex_index,
            2 + last_vertex_index,
            2 + last_vertex_index,
            1 + last_vertex_index,
            3 + last_vertex_index,
        ]);
        self.vertices.reserve(4);
        self.push_vertex(position0, color, [0f32, 0f32], mult, wash, fill);
        self.push_vertex(position1, color, [1f32, 0f32], mult, wash, fill);
        self.push_vertex(position2, color, [0f32, 1f32], mult, wash, fill);
        self.push_vertex(position3, color, [1f32, 1f32], mult, wash, fill);
        self.current_batch().elements += 2;
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
}

pub struct DrawBatch {
    offset: i64,
    elements: i64,
}
