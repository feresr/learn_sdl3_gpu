mod game_dll;

use common::game_memory::GameMemory;
use common::input::keyboard::Keyboard;
use common::input::mouse::Mouse;
use sdl3::event::{Event, WindowEvent};
use sdl3::gpu::ShaderFormat;
use sdl3::keyboard::Keycode;
use std::time::{Duration, Instant};

extern crate nalgebra_glm as glm;

use crate::game_dll::GameDll;

use common::graphics::batch::Batch;
use common::graphics::material::Material;
use common::graphics::render_target::RenderTarget;

pub const FPS: u64 = 60;
pub const FRAME_DURATION: Duration = Duration::from_nanos(1_000_000_000 / FPS);

fn main() {
    // Give us access to windowing and input events
    let sdl_context = sdl3::init().unwrap();

    // Windowing
    let video_subsystem = sdl_context.video().expect("Unable to get video subsystem");
    let window = video_subsystem
        .window("Game", 320 * 4, 180 * 4)
        .resizable()
        .build()
        .expect("Unable to create window");

    // Inputs
    let mut keyboard = Keyboard::default();
    let mut mouse = Mouse::default();
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
    let mut game_memory = GameMemory::default();
    let mut gamedll = GameDll::load();
    let mut screen_target = RenderTarget::empty();

    'running: loop {
        let start = Instant::now();
        keyboard.clear_pressed();
        for event in event_pump.poll_iter() {
            match event {
                Event::Window {
                    timestamp: _,
                    window_id: _,
                    win_event: WindowEvent::Resized(width, height),
                } =>  screen_target.resize(width, height),
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
                    println!("Game DLL reloaded");
                    gamedll = GameDll::load();
                }
                Event::KeyDown {
                    keycode: Some(kc), ..
                } => keyboard.press(kc.clone()),
                Event::KeyUp {
                    keycode: Some(kc), ..
                } => keyboard.release(&kc),
                Event::MouseButtonDown { mouse_btn, .. } => mouse.mouse_button_down(mouse_btn),
                Event::MouseButtonUp { mouse_btn, .. } => mouse.mouse_button_up(mouse_btn),
                Event::MouseMotion {
                    x, y, xrel, yrel, ..
                } => mouse.set_position(x, y, xrel, yrel),
                _ => {}
            }
        }
        let mut cmd = device.acquire_command_buffer().unwrap();
        let texture = cmd.wait_and_acquire_swapchain_texture(&window).unwrap();

        screen_target.set_texture(texture);
        gamedll.update(
            &mut game_memory,
            &mut batch,
            &screen_target,
            &keyboard,
            &mouse,
            &device,
        );
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
