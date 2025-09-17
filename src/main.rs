mod game_dll;
mod graphics;

use common::game_memory::GameMemory;
use sdl3::event::{Event, WindowEvent};
use sdl3::gpu::ShaderFormat;
use sdl3::keyboard::Keycode;
use std::time::{Duration, Instant};

extern crate nalgebra_glm as glm;

use crate::game_dll::GameDll;
use crate::graphics::IDENTITY;
use crate::graphics::batch::Batch;
use crate::graphics::material::Material;
use crate::graphics::render_target::RenderTarget;
use crate::graphics::texture::Texture;

pub const FPS: u64 = 60;
pub const FRAME_DURATION: Duration = Duration::from_nanos(1_000_000_000 / FPS);

fn main() {
    // Give us access to windowing and input events
    let sdl_context = sdl3::init().unwrap();

    // Windowing
    let video_subsystem = sdl_context.video().expect("Unable to get video subsystem");
    let window = video_subsystem
        .window("Game", 320 * 4, 180 * 4)
        .build()
        .expect("Unable to create window");

    // Inputs
    let mut event_pump = sdl_context.event_pump().expect("Unable to get event pump");

    // GPU
    let device = sdl3::gpu::Device::new(
        // TODO: Support more shader formats.
        ShaderFormat::MSL | ShaderFormat::SPIRV,
        cfg!(debug_assertions),
    )
    .expect("Unable to create GPU device")
    .with_window(&window) // Attach to window
    .expect("Unable to attach GPU device to window");

    let mut batch = Batch::new(device.clone(), Material::default(device.clone(), &window));
    let dummy_texture = Texture::from_path(
        device.clone(),
        "/Users/feresr/Workspace/learn_sdl3_gpu/src/atlas-normal.png",
    );

    let mut game_memory = GameMemory::default();
    let mut gamedll = GameDll::load();
    let mut screen_target = RenderTarget::empty();
    let offscreen_target = RenderTarget::new(Texture::new(
        device.clone(),
        320,
        180,
        sdl3::gpu::TextureFormat::R8g8b8a8Unorm,
        // device.get_swapchain_texture_format(&window), // TODO: Texture format
    ));

    let mut game_to_screen_matrix: glm::Mat4;
    {
        let mut cmd = device.acquire_command_buffer().unwrap();
        let texture = cmd.wait_and_acquire_swapchain_texture(&window).unwrap();
        screen_target.set_texture(texture);
        game_to_screen_matrix = create_game_to_screen_target(&screen_target, &offscreen_target);
        screen_target.clear_texture();
        cmd.submit().unwrap();
    }

    'running: loop {
        let start = Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Window {
                    timestamp: _,
                    window_id: _,
                    win_event: WindowEvent::Resized(width, height),
                } => {
                    screen_target.resize(width, height);
                    game_to_screen_matrix =
                        create_game_to_screen_target(&screen_target, &offscreen_target);
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    // Hot-reload the game .dll
                    gamedll = GameDll::load();
                }
                _ => {}
            }
        }

        let mut cmd = device.acquire_command_buffer().unwrap();
        let texture = cmd.wait_and_acquire_swapchain_texture(&window).unwrap();
        screen_target.set_texture(texture);

        // Draw to offscreen target
        {
            gamedll.update(&mut game_memory);
            batch.texture(dummy_texture.clone(), glm::vec2(0.0f32, 0.0f32));
            batch.circle([25.0f32, 90.0f32], 14.0f32, 54, [55, 0, 255, 255]);
            batch.draw(&offscreen_target);
            batch.clear();
        }

        // Draw to window
        {
            batch.push_matrix(game_to_screen_matrix);
            batch.texture(offscreen_target.color(), glm::vec2(0f32, 0f32));
            batch.pop_matrix();
            batch.draw(&screen_target);
            batch.clear();
        }

        screen_target.clear_texture();
        cmd.submit().unwrap();

        precise_sleep(start);
    }

    fn precise_sleep(start: Instant) {
        let sleep_until = start + FRAME_DURATION;
        let now = Instant::now();
        if now < sleep_until {
            let sleep_duration = sleep_until - now;

            // Sleep most of the time, but not all (wake up 1 milli early)
            if sleep_duration > Duration::from_millis(1) {
                std::thread::sleep(sleep_duration - Duration::from_millis(1));
            }

            // Spin for the remaining time for precision
            while Instant::now() < sleep_until {
                std::hint::spin_loop();
            }
        }
    }
}

pub fn create_game_to_screen_target(
    screen_target: &RenderTarget,
    offscreen_target: &RenderTarget,
) -> glm::Mat4 {
    let scale = (screen_target.width as f32 / offscreen_target.width as f32)
        .min(screen_target.height as f32 / offscreen_target.height as f32);

    let screen_center: glm::Vec2 = glm::vec2(
        screen_target.width as f32 / 2f32,
        screen_target.height as f32 / 2f32,
    );
    let game_center: glm::Vec2 = glm::vec2(
        offscreen_target.width as f32 / 2f32,
        offscreen_target.height as f32 / 2f32,
    );

    let game_to_screen_matrix: glm::Mat4 =
        create_transform(screen_center, game_center, glm::vec2(scale, scale));
    return game_to_screen_matrix;
}

// TODO: Find a better place for this
// TODO: Make translation in place
pub fn create_transform(position: glm::Vec2, origin: glm::Vec2, scale: glm::Vec2) -> glm::Mat4 {
    return glm::translate(&IDENTITY, &glm::vec3(position.x, position.y, 0.0f32))
        * glm::scale(&IDENTITY, &glm::vec3(scale.x, scale.y, 1.0f32))
        * glm::translate(&IDENTITY, &glm::vec3(-origin.x, -origin.y, 0.0f32));
}
