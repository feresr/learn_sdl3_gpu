mod game_dll;
mod graphics;

use common::game_memory::GameMemory;
use sdl3::event::Event;
use sdl3::gpu::{ColorTargetInfo, LoadOp, ShaderFormat, StoreOp};
use sdl3::keyboard::Keycode;
use std::time::{Duration, Instant};

use crate::game_dll::GameDll;
use crate::graphics::batch::Batch;
use crate::graphics::material::Material;

pub const FPS: u64 = 60;
pub const FRAME_DURATION: Duration = Duration::from_nanos(1_000_000_000 / FPS);

fn main() {
    // Give us access to windowing and input events
    let sdl_context = sdl3::init().unwrap();

    // Windowing
    let video_subsystem = sdl_context.video().expect("Unable to get video subsystem");
    let window = video_subsystem
        .window("Game", 800, 600)
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

    batch.triangle(
        [-1.0f32, -1.0f32, 0.0],
        [1.0f32, -1.0f32, 0.0f32],
        [0.0f32, 1.0f32, 0.0f32],
        [0, 255, 255, 255],
    );

    let mut game_memory = GameMemory::default();
    let mut gamedll = GameDll::load();

    'running: loop {
        let start = Instant::now();
        for event in event_pump.poll_iter() {
            match event {
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

        gamedll.update(&mut game_memory);

        let mut cmd = device.acquire_command_buffer().unwrap();
        let texture = cmd.wait_and_acquire_swapchain_texture(&window).unwrap();
        let color_target_info = ColorTargetInfo::default()
            .with_texture(&texture)
            .with_store_op(StoreOp::STORE)
            .with_load_op(LoadOp::CLEAR);

        batch.draw(color_target_info);

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
