pub mod batch;
pub mod material;
pub mod mesh;
pub mod render_target;
pub mod subtexture;
pub mod texture;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [u8; 4],
    pub texture_uv: [f32; 2],
    // These unsigned int get interpreted as floats in the fragment shader
    // they are meant to be values between (0 - 255) representing (0.0 to 1.0) in floats
    pub mult_wash_fill: [u8; 4],
}

pub const MAX_VERTICES: u32 = 65536;
pub const MAX_INDICES: u32 = MAX_VERTICES * 3 / 2;
pub static IDENTITY: glm::Mat4 = glm::Mat4::new(
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
);
pub static VEC_2: glm::Vec2 = glm::Vec2::new(0f32, 0f32);
