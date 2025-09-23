use common::{
    Device, Keycode,
    game_memory::GameMemory,
    graphics::{batch::Batch, render_target::RenderTarget},
    input::{
        keyboard::{KEYBOARD, Keyboard},
        mouse::{MOUSE, Mouse},
    },
    ui::{GUI, Gui, widget::Widget},
};

use crate::game_state::{GameState, game_to_screen_projection};

mod game_state;
mod materials;
mod room;

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

    let game_state: &mut GameState = unsafe { &mut *(game_memory.storage as *mut GameState) };

    // TODO: Use this approach with Device?
    unsafe {
        KEYBOARD = keyboard as *const Keyboard;
        MOUSE = mouse as *const Mouse;
        GUI = &mut game_state.gui as *mut Gui;
    }

    // TODO: Avoid re-creating this on very frame.
    let game_to_screen_projection =
        game_to_screen_projection(&game_state.game_target, screen_target);
    let mouse_position: glm::Vec2 = Mouse::position();
    let game_mouse_position =
        Mouse::position_projected(&game_to_screen_projection.try_inverse().unwrap());

    {
        let window = Gui::window("Mouse data");
        game_state.dummy_string.clear();
        game_state.dummy_string.push_str(&format!(
            "Position x:{:.1} y:{:.1}",
            mouse_position.x, mouse_position.y
        ));
        window.add_widget(Widget::TEXT(game_state.dummy_string.clone()));
    }

    if Keyboard::held(Keycode::A) {
        game_state.dummy_position.x -= 1.0f32;
    }
    if Keyboard::held(Keycode::D) {
        game_state.dummy_position.x += 1.0f32;
    }

    // Draw to low-res off-screen game target
    {
        // Draw foreground tiles (TODO: Render to an offscreen target only once - composite target)
        for (x, y, _) in &game_state.room.foreground_tiles {
            let sprite = game_state.atlas.get(y as u16 % 3, (x + y) as u16 % 3);
            batch.subtexture(sprite, glm::vec2(x as f32 * 8f32, y as f32 * 8f32));
        }

        batch.push_material(&game_state.material);

        batch.circle(
            [
                game_mouse_position.x as i32 as f32,
                game_mouse_position.y as i32 as f32,
            ],
            14.0f32,
            54,
            [255, 255, 255, 255],
        );
        batch.pop_material();

        batch.draw_into(&game_state.game_target);

        // Add debug informatoin
        {
            let window = Gui::window("Game Offscreen Target");
            window.add_widget(Widget::TEXT("Example test text 123456789".to_string()));
            if window.add_widget(Widget::BUTTON("Click me!", [80, 29, 175, 255])) {
                game_state.dummy_bool = !game_state.dummy_bool;
            }
            window.add_widget(Widget::TEXTURE(game_state.game_target.color()));
            batch.debug(window);
        }

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

        let window = Gui::window("Screen Draw");
        batch.debug(window);

        Gui::draw(batch);

        batch.draw_into(&screen_target);
        batch.clear();
    }
    // game_state.arena.reset();
}
