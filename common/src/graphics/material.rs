use sdl3::{
    gpu::{
        BlendFactor, ColorTargetBlendState, ColorTargetDescription, Device, GraphicsPipeline,
        GraphicsPipelineTargetInfo, PrimitiveType, Shader, ShaderFormat, ShaderStage,
        TextureFormat, VertexAttribute, VertexBufferDescription, VertexElementFormat,
        VertexInputRate, VertexInputState,
    },
    video::Window,
};
use std::ffi::CStr;

use crate::graphics::Vertex;

static FS_ENTRY: &CStr = c"fragment_main";
static VS_ENTRY: &CStr = c"vertex_main";

#[derive(Clone)]
pub struct Material {
    pub pipeline: GraphicsPipeline,
    pub target_texture_format: TextureFormat,
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.pipeline.raw() == other.pipeline.raw()
            && self.target_texture_format == other.target_texture_format
    }
}

static DEFAULT_SHADER_FRAGMENT_SRC: &str = include_str!("../shaders/compiled/default.fragment.msl");
static DEFAULT_SHADER_VERTEX_SRC: &str = include_str!("../shaders/compiled/default.vertex.msl");

impl Material {
    pub fn default(device: Device, window: &Window) -> Self {
        let supported_formats_bitflag = device.get_shader_formats();

        let shader_format;
        if (supported_formats_bitflag & ShaderFormat::MSL) == ShaderFormat::MSL {
            shader_format = ShaderFormat::MSL
        } else {
            panic!("Shader format not supported")
        }

        let fs: Shader = device
            .create_shader()
            .with_samplers(1) // Texture
            .with_code(
                shader_format,
                DEFAULT_SHADER_FRAGMENT_SRC.as_bytes(),
                ShaderStage::Fragment,
            )
            .with_entrypoint(FS_ENTRY)
            .build()
            .expect("Unable to create fragment shader");

        let vs: Shader = device
            .create_shader()
            .with_uniform_buffers(1) // Projection Matrix
            .with_code(
                shader_format,
                DEFAULT_SHADER_VERTEX_SRC.as_bytes(),
                ShaderStage::Vertex,
            )
            .with_entrypoint(VS_ENTRY)
            .build()
            .expect("Unable to create vertex shader");

        let texture_format = device.get_swapchain_texture_format(&window);

        return Self::new(device, vs, fs, texture_format);
    }

    pub fn new(
        device: Device,
        vs: Shader,
        fs: Shader,
        target_texture_format: TextureFormat,
    ) -> Self {
        let graphics_pipeline = device
            .create_graphics_pipeline()
            .with_target_info(
                GraphicsPipelineTargetInfo::new().with_color_target_descriptions(&[
                    ColorTargetDescription::new()
                        .with_blend_state(
                            ColorTargetBlendState::new()
                                .with_src_color_blendfactor(BlendFactor::SrcAlpha)
                                .with_dst_color_blendfactor(BlendFactor::OneMinusSrcAlpha)
                                .with_src_alpha_blendfactor(BlendFactor::One)
                                .with_dst_alpha_blendfactor(BlendFactor::OneMinusSrcAlpha)
                                .with_alpha_blend_op(sdl3::gpu::BlendOp::Add)
                                .with_color_blend_op(sdl3::gpu::BlendOp::Add)
                                .with_enable_blend(true),
                        )
                        .with_format(target_texture_format),
                ]),
            )
            .with_primitive_type(PrimitiveType::TriangleList)
            .with_vertex_input_state(
                VertexInputState::new()
                    .with_vertex_buffer_descriptions(&[VertexBufferDescription::new()
                        .with_slot(0)
                        .with_input_rate(VertexInputRate::Vertex)
                        .with_pitch(size_of::<Vertex>() as u32)
                        .with_instance_step_rate(0)])
                    .with_vertex_attributes(&[
                        VertexAttribute::new()
                            .with_buffer_slot(0)
                            .with_offset(0)
                            .with_location(0)
                            .with_format(VertexElementFormat::Float3), // position
                        VertexAttribute::new()
                            .with_buffer_slot(0)
                            .with_offset(size_of::<f32>() as u32 * 3)
                            .with_location(1)
                            .with_format(VertexElementFormat::Ubyte4Norm), // color
                        VertexAttribute::new()
                            .with_buffer_slot(0)
                            .with_offset(
                                (size_of::<f32>() as u32 * 3) + (size_of::<u8>() as u32 * 4),
                            )
                            .with_location(2)
                            .with_format(VertexElementFormat::Float2), // texture_uv
                        VertexAttribute::new()
                            .with_buffer_slot(0)
                            .with_offset(
                                (size_of::<f32>() as u32 * 5) + (size_of::<u8>() as u32 * 4),
                            )
                            .with_location(3)
                            .with_format(VertexElementFormat::Ubyte4Norm), // color
                    ]),
            )
            .with_vertex_shader(&vs)
            .with_fragment_shader(&fs)
            .build()
            .unwrap();

        return Material {
            pipeline: graphics_pipeline,
            target_texture_format: target_texture_format,
        };
    }

    pub fn from_specification(device: Device, specification: &MaterialSpecification) -> Self {
        let supported_formats_bitflag = device.get_shader_formats();
        let shader_format;
        if (supported_formats_bitflag & ShaderFormat::MSL) == ShaderFormat::MSL {
            shader_format = ShaderFormat::MSL
        } else {
            panic!("Shader format not supported")
        }

        let fs: Shader = device
            .create_shader()
            .with_samplers(specification.fragment.sampler_count) // Texture
            .with_uniform_buffers(specification.fragment.uniform_buffer_count)
            .with_code(
                shader_format,
                specification.fragment.src.as_bytes(),
                ShaderStage::Fragment,
            )
            .with_entrypoint(FS_ENTRY)
            .build()
            .expect("Unable to create fragment shader");

        let vs: Shader = device
            .create_shader()
            .with_samplers(specification.vertex.sampler_count)
            .with_uniform_buffers(specification.vertex.uniform_buffer_count) // Projection Matrix
            .with_code(
                shader_format,
                specification.vertex.src.as_bytes(),
                ShaderStage::Vertex,
            )
            .with_entrypoint(VS_ENTRY)
            .build()
            .expect("Unable to create vertex shader");

        return Self::new(device, vs, fs, specification.texture_format);
    }
}

pub struct ShaderSpecification {
    pub src: &'static str,
    pub uniform_buffer_count: u32,
    pub sampler_count: u32,
}

pub struct MaterialSpecification {
    pub fragment: ShaderSpecification,
    pub vertex: ShaderSpecification,
    pub texture_format: TextureFormat,
}
