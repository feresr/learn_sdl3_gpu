use sdl3::{
    gpu::{
        ColorTargetDescription, Device, GraphicsPipeline, GraphicsPipelineTargetInfo,
        PrimitiveType, Shader, ShaderFormat, ShaderStage, TextureFormat, VertexAttribute,
        VertexBufferDescription, VertexElementFormat, VertexInputRate, VertexInputState,
    },
    video::Window,
};
use std::ffi::CString;
use std::fs;

use crate::graphics::Vertex;

pub struct Material {
    pub pipeline: GraphicsPipeline,
    pub target_texture_format: TextureFormat,
}

impl Material {
    pub fn default(device: Device, window: &Window) -> Self {
        let supported_formats_bitflag = device.get_shader_formats();

        let shader_format;
        if (supported_formats_bitflag & ShaderFormat::MSL) == ShaderFormat::MSL {
            shader_format = ShaderFormat::MSL
        } else {
            panic!("Shader format not supported")
        }

        // TODO: Use SDL to read string?
        let vs = fs::read_to_string("src/shaders/compiled/triangle.vertex.msl").unwrap();
        let fs = fs::read_to_string("src/shaders/compiled/triangle.fragment.msl").unwrap();

        let fs_entry = CString::new("fragment_main").unwrap();
        let vs_entry = CString::new("vertex_main").unwrap();

        let fs = device
            .create_shader()
            .with_code(shader_format, fs.as_bytes(), ShaderStage::Fragment)
            .with_entrypoint(fs_entry.as_c_str())
            .build()
            .expect("Unable to create fragment shader");

        let vs = device
            .create_shader()
            .with_code(shader_format, vs.as_bytes(), ShaderStage::Vertex)
            .with_entrypoint(vs_entry.as_c_str())
            .build()
            .expect("Unable to create vertex shader");

        let format = device.get_swapchain_texture_format(&window);

        return Self::new(device, vs, fs, format);
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
                    ColorTargetDescription::new().with_format(target_texture_format),
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
    
}
