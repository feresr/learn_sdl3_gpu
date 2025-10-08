use std::f32::consts::TAU;
use std::fmt::Debug;

use sdl3::gpu::{BufferBinding, Device};

use crate::graphics::material::Material;
use crate::graphics::mesh::Mesh;
use crate::graphics::render_target::RenderTarget;
use crate::graphics::subtexture::Subtexture;
use crate::graphics::texture::Texture;
use crate::graphics::{IDENTITY, Vertex};

pub struct Batch {
    device: Device,
    mesh: Mesh,
    default_material: Material,
    // TODO: All these Vec will allocate dynamically repace with array or pre-allocate them?
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    matrix_stack: Vec<glm::Mat4>,
    material_stack: Vec<Material>,
    batches: Vec<DrawBatch>,
}

impl Debug for Batch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Batch")
            .field("vertices", &self.vertices.len())
            .field("indices", &self.indices)
            .field("batch count:", &self.batches.len())
            .finish()
    }
}

impl Batch {
    pub fn new(device: Device, default_material: Material) -> Self {
        Batch {
            device: device.clone(),
            mesh: Mesh::new(device),
            default_material: default_material,
            vertices: Default::default(),
            indices: Default::default(),
            matrix_stack: Default::default(),
            material_stack: Default::default(),
            batches: Default::default(),
        }
    }

    pub fn push_material(&mut self, material: &Material) {
        let current_material = self.current_batch().material.clone();
        self.material_stack.push(current_material);
        let current: &mut DrawBatch = self.current_batch();
        if current.elements > 0 && *material != current.material {
            self.push_batch();
        }
        self.current_batch().material = material.clone();
    }

    pub fn pop_material(&mut self) -> Material {
        let material = self.material_stack.pop().unwrap();
        let current: &mut DrawBatch = self.current_batch();
        if current.elements > 0 && material != current.material {
            self.push_batch();
        }
        self.current_batch().material = material.clone();
        return material;
    }

    pub fn push_matrix(&mut self, matrix: glm::Mat4) {
        if self.matrix_stack.is_empty() {
            self.matrix_stack.push(matrix);
        } else {
            let current: &glm::Mat4 = self.peek_matrix();
            self.matrix_stack.push(current * matrix);
        }
    }

    pub fn peek_matrix(&self) -> &glm::Mat4 {
        if self.matrix_stack.is_empty() {
            return &IDENTITY;
        } else {
            return &self.matrix_stack.last().unwrap();
        }
    }

    pub fn pop_matrix(&mut self) {
        self.matrix_stack.pop();
    }

    fn push_batch(&mut self) {
        let current = self.current_batch();
        let value = DrawBatch {
            offset: current.offset + current.elements,
            elements: 0,
            material: current.material.clone(),
            texture: current.texture.clone(),
            ..*current
        };
        self.batches.push(value);
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

    // TODO: why is this taking a glm::2 as position (and not [f32;3])
    pub fn texture(&mut self, texture: Texture, position: &glm::Vec2) {
        let mut current_batch = self.current_batch();

        // Push a new batch if the current_batch already has a texture assigned.
        if let Some(batch_texture) = current_batch.texture.as_ref() {
            if batch_texture != &texture {
                self.push_batch();
                current_batch = self.current_batch();
            }
        }

        let position0 = [position.x, position.y, 0.0f32];
        let position1 = [position.x + texture.width() as f32, position.y, 0.0f32];
        let position2 = [position.x, position.y + texture.height() as f32, 0.0f32];
        let position3 = [
            position.x + texture.width() as f32,
            position.y + texture.height() as f32,
            0.0f32,
        ];

        current_batch.texture = Some(texture);

        self.push_quad(
            position0,
            position1,
            position2,
            position3,
            [0f32, 0f32],
            [1f32, 0f32],
            [0f32, 1f32],
            [1f32, 1f32],
            255,
            0,
            0,
            [255, 255, 255, 255],
        );
    }

    pub fn subtexture(&mut self, subtexture: Subtexture, position: glm::Vec2) {
        let mut current_batch = self.current_batch();
        if let Some(batch_texture) = current_batch.texture.as_ref() {
            if batch_texture != &subtexture.texture {
                self.push_batch();
                current_batch = self.current_batch();
            }
        }
        current_batch.texture = Some(subtexture.texture);

        let position0 = [position.x, position.y, 0.0f32];
        let position1 = [position.x + subtexture.rect.w as f32, position.y, 0.0f32];
        let position2 = [position.x, position.y + subtexture.rect.h as f32, 0.0f32];
        let position3 = [
            position.x + subtexture.rect.w as f32,
            position.y + subtexture.rect.h as f32,
            0.0f32,
        ];
        let uvs = subtexture.uvs;
        self.push_quad(
            position0,
            position1,
            position2,
            position3,
            [uvs.x, uvs.y],
            [uvs.x + uvs.w, uvs.y],
            [uvs.x, uvs.y + uvs.h],
            [uvs.x + uvs.w, uvs.y + uvs.h],
            255,
            0,
            0,
            [255, 255, 255, 255],
        );
    }

    pub fn rect(&mut self, position: [f32; 3], size: [f32; 2], color: [u8; 4]) {
        // Top-left -> Top-right -> Bottom-left -> Bottom-right
        self.quad(
            position,
            [position[0] + size[0], position[1], position[2]],
            [position[0], position[1] + size[1], position[2]],
            [position[0] + size[0], position[1] + size[1], position[2]],
            color,
        );
    }

    pub fn rect_outline(
        &mut self,
        position: [f32; 3],
        size: [f32; 2],
        color: [u8; 4],
        thickness: f32,
    ) {
        let top_left = position;
        let top_right = [position[0] + size[0], position[1], position[2]];
        let bottom_left = [position[0], position[1] + size[1], position[2]];
        let bottom_right = [position[0] + size[0], position[1] + size[1], position[2]];

        // TOP edge
        self.quad(
            [ top_left[0] - thickness / 2f32,
                top_left[1] - thickness / 2f32,
                top_left[2],
            ],
            [
                top_right[0] + thickness / 2f32,
                top_right[1] - thickness / 2f32,
                top_right[2],
            ],
            [
                top_left[0] - thickness / 2f32,
                top_left[1] + thickness / 2f32,
                top_left[2],
              ],
            [
                top_right[0] + thickness / 2f32,
                top_right[1] + thickness / 2f32,
                top_right[2],
            ],
            color,
        );
        // BOTTOM edge
        self.quad(
            [
                bottom_left[0] - thickness / 2f32,
                bottom_left[1] - thickness / 2f32,
                bottom_left[2],
            ],
            [
                bottom_right[0] + thickness / 2f32,
                bottom_right[1] - thickness / 2f32,
                bottom_right[2],
            ],
            [
                bottom_left[0] - thickness / 2f32,
                bottom_left[1] + thickness / 2f32,
                bottom_left[2],
              ],
            [
                bottom_right[0] + thickness / 2f32,
                bottom_right[1] + thickness / 2f32,
                bottom_right[2],
            ],
            color,
        );
        // LEFT edge
        self.quad(
            [ 
                top_left[0] - thickness / 2f32,
                top_left[1] - thickness / 2f32,
                top_left[2],
            ],
            [
                top_left[0] + thickness / 2f32,
                top_left[1] - thickness / 2f32,
                top_left[2],
            ],
            [
                bottom_left[0] - thickness / 2f32,
                bottom_left[1] + thickness / 2f32,
                bottom_left[2],
              ],
            [
                bottom_left[0] + thickness / 2f32,
                bottom_left[1] + thickness / 2f32,
                bottom_left[2],
            ],
            color,
        );
        // Right edge
        self.quad(
            [ 
                top_right[0] - thickness / 2f32,
                top_right[1] - thickness / 2f32,
               top_right[2],
            ],
            [
                top_right[0] + thickness / 2f32,
                top_right[1] - thickness / 2f32,
                top_right[2],
            ],
            [
                bottom_right[0] - thickness / 2f32,
                bottom_right[1] + thickness / 2f32,
                bottom_right[2],
              ],
            [
                bottom_right[0] + thickness / 2f32,
                bottom_right[1] + thickness / 2f32,
                bottom_right[2]], color
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
        self.push_quad(
            position0,
            position1,
            position2,
            position3,
            [0f32, 0f32],
            [1f32, 0f32],
            [0f32, 1f32],
            [1f32, 1f32],
            0,
            0,
            255,
            color,
        );
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

    pub fn draw_into(&mut self, target: &RenderTarget) {
        // println!("{:#?}", self);

        // Copy pass
        {
            let upload_cmd = self.device.acquire_command_buffer().unwrap();
            let copy_pass = self.device.begin_copy_pass(&upload_cmd).unwrap();
            for batch in &mut self.batches {
                if let Some(texture) = batch.texture.as_mut() {
                    texture.upload(&copy_pass);
                }
            }

            self.mesh.set_data(&self.vertices);
            self.mesh.set_indices(&self.indices);
            self.mesh.upload(&copy_pass);

            self.device.end_copy_pass(copy_pass);
            upload_cmd.submit().unwrap();
        }

        // Render pass
        {
            let render_cmd = self.device.acquire_command_buffer().unwrap();
            let render_pass = self
                .device
                .begin_render_pass(&render_cmd, &[target.color_target_info()], None)
                .unwrap();

            let buffer_binding = BufferBinding::new()
                .with_offset(0)
                .with_buffer(&self.mesh.vertex_buffer);
            let index_binding = BufferBinding::new()
                .with_offset(0)
                .with_buffer(&self.mesh.index_buffer);

            render_cmd.push_vertex_uniform_data(0, target.projection());

            render_pass.bind_vertex_buffers(0, &[buffer_binding]);
            render_pass.bind_index_buffer(
                &index_binding,
                sdl3::sys::gpu::SDL_GPUIndexElementSize::_32BIT,
            );

            for batch in &self.batches {
                if batch.elements == 0 {
                    // TODO: Is adding an empty batch needed?
                    break;
                }
                if let Some(texture) = &batch.texture {
                    render_pass.bind_fragment_samplers(0, &[texture.bindings()]);
                }
                render_pass.bind_graphics_pipeline(&batch.material.pipeline);
                render_pass.draw_indexed_primitives(
                    (batch.elements * 3) as u32,
                    1,
                    (batch.offset * 3) as u32,
                    0,
                    0,
                );
            }

            self.device.end_render_pass(render_pass);
            render_cmd.submit().unwrap();
        }
    }

    fn push_quad(
        &mut self,
        position0: [f32; 3],
        position1: [f32; 3],
        position2: [f32; 3],
        position3: [f32; 3],
        uv0: [f32; 2],
        uv1: [f32; 2],
        uv2: [f32; 2],
        uv3: [f32; 2],
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
        self.push_vertex(position0, color, uv0, mult, wash, fill);
        self.push_vertex(position1, color, uv1, mult, wash, fill);
        self.push_vertex(position2, color, uv2, mult, wash, fill);
        self.push_vertex(position3, color, uv3, mult, wash, fill);
        self.current_batch().elements += 2;
    }

    fn current_batch(&mut self) -> &mut DrawBatch {
        if self.batches.is_empty() {
            let value = DrawBatch {
                offset: 0,
                elements: 0,
                material: self.default_material.clone(),
                texture: None,
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
        let matrix: &glm::Mat4 = self.peek_matrix();
        let projected: glm::Vec3 =
            (matrix * glm::vec4(position[0], position[1], position[2], 1.0)).xyz();
        self.vertices.push(Vertex {
            position: projected.into(),
            color,
            texture_uv,
            mult_wash_fill: [mult, wash, fill, 0],
        });
    }

    pub fn get_batch_count(&self) -> usize {
        self.batches.iter().filter(|b| b.elements > 0).count()
    }

    pub fn clear(&mut self) {
        self.batches.clear();
        self.vertices.clear();
        self.indices.clear();
        self.matrix_stack.clear();
        self.material_stack.clear();
    }
}

pub struct DrawBatch {
    offset: i64,
    elements: i64,
    material: Material,
    texture: Option<Texture>,
}
