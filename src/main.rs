use sdl3::event::Event;
use sdl3::gpu::{
    BufferBinding, BufferRegion, BufferUsageFlags, ColorTargetDescription, ColorTargetInfo, Filter,
    GraphicsPipelineTargetInfo, LoadOp, PrimitiveType, SamplerAddressMode, SamplerCreateInfo,
    SamplerMipmapMode, ShaderFormat, ShaderStage, StoreOp, TextureCreateInfo, TextureFormat,
    TextureRegion, TextureSamplerBinding, TextureTransferInfo, TextureType, TextureUsage,
    TransferBufferLocation, TransferBufferUsage, VertexAttribute, VertexBufferDescription,
    VertexElementFormat, VertexInputRate, VertexInputState,
};
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use stb_image::image::LoadResult;
use std::ffi::CString;
use std::fs;
use std::time::Duration;

#[repr(C)]
#[derive(Copy, Clone)]
struct PositionColorVertex {
    position: [f32; 3],
    color: [f32; 3],
}

fn main() {
    // Give us access to windowing and input events
    let sdl_context = sdl3::init().unwrap();
    dbg!(size_of::<PositionColorVertex>());

    // Windowing
    let video_subsystem = sdl_context.video().expect("Unable to get video subsystem");
    let window = video_subsystem
        .window("Hello World", 800, 600)
        .build()
        .expect("Unable to create window");
    // Inputs
    let mut event_pump = sdl_context.event_pump().expect("Unable to get event pump");

    // GPU
    let device = sdl3::gpu::Device::new(ShaderFormat::Msl | ShaderFormat::SpirV, true)
        .expect("Unable to create GPU device")
        .with_window(&window) // Attach to window
        .expect("Unable to attach GPU device to window");

    let colors = [Color::BLACK, Color::GREEN, Color::BLUE, Color::RED];
    let mut color_index = 0;
    let shader_format = ShaderFormat::Msl; //device.get_shader_formats();

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

    // ---- Texture ----

    let sampler = device
        .create_sampler(
            SamplerCreateInfo::new()
                .with_min_filter(Filter::Linear)
                .with_mag_filter(Filter::Linear)
                .with_mipmap_mode(SamplerMipmapMode::Linear)
                .with_address_mode_u(SamplerAddressMode::Repeat)
                .with_address_mode_v(SamplerAddressMode::Repeat)
                .with_address_mode_w(SamplerAddressMode::Repeat),
        )
        .unwrap();

    let load_result = stb_image::image::load("src/smb.png");
    let image = match load_result {
        LoadResult::Error(e) => {
            panic!("Error loading image {}", e.as_str());
        }
        LoadResult::ImageU8(img) => img,
        LoadResult::ImageF32(img) => {
            panic!("Error loading image f32");
        }
    };

    let texture = device
        .create_texture(
            TextureCreateInfo::new()
                .with_type(TextureType::_2D)
                .with_format(TextureFormat::R8g8b8a8Unorm)
                .with_layer_count_or_depth(1)
                .with_num_levels(1)
                .with_usage(TextureUsage::Sampler)
                .with_width(image.width as u32)
                .with_height(image.height as u32),
        )
        .unwrap();

    let texture_transfer_buffer = device
        .create_transfer_buffer()
        .with_size(image.data.len() as u32)
        .with_usage(TransferBufferUsage::Upload)
        .build()
        .unwrap();

    let mut mem_map = texture_transfer_buffer.map(&device, true);
    mem_map.mem_mut().copy_from_slice(&image.data);
    mem_map.unmap();

    let upload_cmd = device.acquire_command_buffer().unwrap();
    let copy_pass = device.begin_copy_pass(&upload_cmd).unwrap();

    copy_pass.upload_to_gpu_texture(
        TextureTransferInfo::new()
            .with_transfer_buffer(&texture_transfer_buffer)
            .with_pixels_per_row(image.width as u32)
            .with_rows_per_layer(image.height as u32)
            .with_offset(0),
        TextureRegion::new()
            .with_depth(image.depth as u32)
            .with_width(image.width as u32)
            .with_height(image.height as u32)
            .with_texture(&texture),
        false,
    );
    device.end_copy_pass(copy_pass);
    upload_cmd.submit().unwrap();

    // ----- End Texture ----

    let graphics_pipeline = device
        .create_graphics_pipeline()
        .with_target_info(
            GraphicsPipelineTargetInfo::new()
                .with_color_target_descriptions(&[ColorTargetDescription::new()
                    .with_format(device.get_swapchain_texture_format(&window))]),
        )
        .with_primitive_type(PrimitiveType::TriangleList)
        .with_vertex_input_state(
            VertexInputState::new()
                .with_vertex_buffer_descriptions(&[VertexBufferDescription::new()
                    .with_slot(0)
                    .with_input_rate(VertexInputRate::Vertex)
                    .with_pitch(size_of::<PositionColorVertex>() as u32)
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
                        .with_format(VertexElementFormat::Float3), // color
                ]),
        )
        .with_vertex_shader(&vs)
        .with_fragment_shader(&fs)
        .build()
        .unwrap();

    let buffer = device
        .create_buffer()
        .with_usage(BufferUsageFlags::Vertex)
        .with_size(size_of::<PositionColorVertex>() as u32 * 3u32)
        .build()
        .unwrap();

    let transfer_buffer = device
        .create_transfer_buffer()
        .with_usage(TransferBufferUsage::Upload)
        .with_size(size_of::<PositionColorVertex>() as u32 * 3u32)
        .build()
        .unwrap();

    let mut position = [0.0, 0.0, 0.0];

    upload_data(&transfer_buffer, &buffer, &device, position, [1.0, 0.0]);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    color_index = (color_index + 1) % colors.len();
                    position[0] += 0.1;
                    upload_data(&transfer_buffer, &buffer, &device, position, [0.0, 1.0]);
                }
                _ => {}
            }
        }

        let mut cmd = device.acquire_command_buffer().unwrap();
        let swap_chain_texture = cmd.wait_and_acquire_swapchain_texture(&window).unwrap();
        let color_target_info = ColorTargetInfo::default()
            .with_texture(&swap_chain_texture)
            .with_clear_color(colors[color_index])
            .with_store_op(StoreOp::Store)
            .with_load_op(LoadOp::Clear);

        let render_pass = device
            .begin_render_pass(&cmd, &[color_target_info], None)
            .unwrap();

        // Rendering happens here
        render_pass.bind_graphics_pipeline(&graphics_pipeline);
        let buffer_binding = BufferBinding::new().with_offset(0).with_buffer(&buffer);
        let sampler_binding = TextureSamplerBinding::new()
            .with_sampler(&sampler)
            .with_texture(&texture);
        render_pass.bind_vertex_buffers(0, &[buffer_binding]);
        render_pass.bind_fragment_samplers(0, &[sampler_binding]);
        render_pass.draw_primitives(3, 1, 0, 0);

        device.end_render_pass(render_pass);

        cmd.submit().unwrap();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

// const colors : [[f32;3];4] = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

fn upload_data(
    transfer_buffer: &sdl3::gpu::TransferBuffer,
    dst_buffer: &sdl3::gpu::Buffer,
    device: &sdl3::gpu::Device,
    position: [f32; 3],
    color: [f32; 2],
) {
    let mut mem_map = transfer_buffer.map(&device, false);
    let memory = mem_map.mem_mut();
    memory[0] = PositionColorVertex {
        position,
        color: [0.0, 0.0, 0.0],
    };
    memory[1] = PositionColorVertex {
        position: [1.0, -1.0, 0.0],
        color: [0.0, 1.0, 0.0],
    };
    memory[2] = PositionColorVertex {
        position: [0.0, 1.0, 0.0],
        color: [1.0, 0.0, 0.0],
    };
    mem_map.unmap();

    let upload_cmd = device.acquire_command_buffer().unwrap();
    let copy_pass = device.begin_copy_pass(&upload_cmd).unwrap();
    copy_pass.upload_to_gpu_buffer(
        TransferBufferLocation::new() // Source
            .with_transfer_buffer(&transfer_buffer)
            .with_offset(0),
        BufferRegion::new() // Destination
            .with_buffer(&dst_buffer)
            .with_offset(0)
            .with_size(size_of::<PositionColorVertex>() as u32 * 3u32),
        true,
    );
    device.end_copy_pass(copy_pass);
    upload_cmd.submit().unwrap();
}
