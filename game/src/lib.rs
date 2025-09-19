use common::{
    Device, Keycode,
    game_memory::GameMemory,
    graphics::{batch::Batch, render_target::RenderTarget},
    input::{
        keyboard::{KEYBOARD, Keyboard},
        mouse::{MOUSE, Mouse},
    },
};

use crate::game_state::{game_to_screen_projection, GameState};

mod game_state;
mod materials;

extern crate nalgebra_glm as glm;

#[unsafe(no_mangle)]
pub extern "C" fn update_game(
    game_memory: &mut GameMemory,
    batch: &mut Batch,
    screen_target: &mut RenderTarget,
    keyboard: &Keyboard,
    mouse: &Mouse,
    device: &Device,
) {
    if !game_memory.initialized {
        debug_assert!(
            std::mem::size_of::<GameState>() <= GameMemory::GAME_MEMORY,
            "GameState ({}) is larger than the available game memory ({})",
            std::mem::size_of::<GameState>(),
            GameMemory::GAME_MEMORY
        );
        unsafe { (game_memory.storage as *mut GameState).write(GameState::new(device.clone())) }
        game_memory.initialized = true;
    }
    // TODO: Doing this every frame might be unnecessary, introduce game_memory.hot_reloaded or similar
    // TODO: Use this approach with Device?
    unsafe {
        KEYBOARD = keyboard as *const Keyboard;
        MOUSE = mouse as *const Mouse;
    }

    let game_state: &mut GameState = unsafe { &mut *(game_memory.storage as *mut GameState) };

    let game_to_screen_projection = game_to_screen_projection(&game_state.game_target, screen_target);
    let mouse_position: glm::Vec2 = Mouse::position();
    let game_mouse_position =
        Mouse::position_projected(&game_to_screen_projection.try_inverse().unwrap());

    if Keyboard::held(Keycode::A) {
        game_state.dummy_position.x -= 1.0f32;
    }
    if Keyboard::held(Keycode::D) {
        game_state.dummy_position.x += 1.0f32;
    }

    // Draw to low-res off-screen game target
    {
        batch.push_material(&game_state.material);

        batch.circle(
            [game_mouse_position.x as i32 as f32, game_mouse_position.y as i32 as f32],
            14.0f32,
            54,
            [255, 255, 255, 255],
        );
        batch.pop_material();

        batch.texture(game_state.dummy_texture.clone(), game_state.dummy_position);
        batch.draw_into(&game_state.game_target);
        batch.clear();
    }

    // Draw to screen window
    {
        batch.push_matrix(game_to_screen_projection);
        batch.texture(game_state.game_target.color(), glm::vec2(0f32, 0f32));
        batch.pop_matrix();
        batch.circle(
            [mouse_position.x, mouse_position.y],
            10.0f32,
            54,
            [255, 255, 255, 255],
        );

        batch.draw_into(&screen_target);
        batch.clear();
    }
}
