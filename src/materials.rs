use sdl3::gpu::TextureFormat;

use crate::graphics::material::{MaterialSpecification, ShaderSpecification};

static RED_SHADER_FRAGMENT_SRC: &str = include_str!("shaders/compiled/red.fragment.msl");
static RED_SHADER_VERTEX_SRC: &str = include_str!("shaders/compiled/red.vertex.msl");

pub const RED_MATERIAL: MaterialSpecification = MaterialSpecification {
    fragment: ShaderSpecification {
        src: RED_SHADER_FRAGMENT_SRC,
        sampler_count: 1,
        uniform_buffer_count: 0,
    },
    vertex: ShaderSpecification {
        src: RED_SHADER_VERTEX_SRC,
        uniform_buffer_count: 1,
        sampler_count: 0,
    },
    texture_format: TextureFormat::R8g8b8a8Unorm,
};
