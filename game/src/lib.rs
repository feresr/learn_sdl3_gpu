use common::{
    Device,
    game_memory::GameMemory,
    graphics::{IDENTITY, VEC_2_ZERO, batch::Batch, render_target::RenderTarget},
    input::{
        keyboard::{KEYBOARD, Keyboard},
        mouse::{MOUSE, Mouse},
    },
    ui::{
        gui::{GUI, Gui},
        widget::Widget,
    },
};

use crate::{
    editor::Editor,
    game::{Game, create_target_projection},
};

mod editor;
mod game;
mod grid;
mod materials;
mod player;
mod room;
mod sprite;
mod world;

// TODO: Custom allocator (allocate only on provided memory)
// Use a bitmap allocator, represet each byte with a bit
// [00100011] -> this means 2 bytes free, one taken, 3 free, 2 taken
// If allocating a byte, just find the first 0 -> index: 0
// If allocating 2 bytes, re-interpret this as chucks of 2. Find the first zero [[00][10][00][11]] -> index 0 (index 2 is also free)
// If allocating 3 bytes, round up to the closest multiple of 4, Find the first zero [[0010][0011]] -> no free space for this element
// De allocating works the same way.

extern crate nalgebra_glm as glm;

pub static mut WINDOW_SIZE: (u32, u32) = (0, 0);

pub static mut GAME_TO_SCREEN_PROJECTION: glm::Mat4 = IDENTITY;
pub static mut SCREEN_TO_GAME_PROJECTION: glm::Mat4 = IDENTITY;

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
            std::mem::size_of::<Game>() <= GameMemory::GAME_MEMORY,
            "GameState ({}) is larger than the available game memory ({})",
            std::mem::size_of::<Game>(),
            GameMemory::GAME_MEMORY
        );
        unsafe {
            // In debug memory is partitioned like: [[Game], [Editor]];
            let game_ptr = game_memory.storage as *mut Game;
            game_ptr.write(Game::new(device.clone()));

            // TODO: Do not write editor in Release
            let editor_ptr = game_ptr.add(1) as *mut Editor;
            editor_ptr.write(Editor::default());

            // TODO: clear memory on game exit (no destructors are being invoked here!)
            // SDL primitives are allocated outside this storage. Consider sdl3_sys SetMemoryFunctions()
            // Game also allocates in the heap (global allocator). Consider replacing this with a custom bitmap allocator
        }
        game_memory.initialized = true;
    }

    let game: &mut Game = unsafe { &mut *(game_memory.storage as *mut Game) };
    let editor: &mut Editor =
        unsafe { &mut *((game_memory.storage as *mut Game).add(1) as *mut Editor) };

    unsafe {
        // dbg!((*ARENA_ALLOC.arena).bytes_used());
        // dbg!((*ARENA_ALLOC.arena).capacity());

        KEYBOARD = keyboard as *const Keyboard;
        MOUSE = mouse as *const Mouse;
        GUI = &mut game.gui as *mut Gui;

        if WINDOW_SIZE.0 != screen_target.width || WINDOW_SIZE.1 != screen_target.height {
            GAME_TO_SCREEN_PROJECTION = create_target_projection(&game.game_target, screen_target);
            SCREEN_TO_GAME_PROJECTION = GAME_TO_SCREEN_PROJECTION.try_inverse().unwrap();
            WINDOW_SIZE.0 = screen_target.width;
            WINDOW_SIZE.1 = screen_target.height;
        }
    }

    Gui::update(); // Detect input before anything else
    batch.clear();

    if editor.is_showing {
        editor.update(&mut game.world, &game.tile_atlas);
        editor.render(batch, &game.world, &game.tile_atlas);
    } else {
        game.update();
        game.render(batch);

        let draw_count = batch.get_batch_count();

        let window = Gui::window("Offscreen targets");
        window.set_direction(common::ui::utils::Direction::Vertical);
        window.add_widget(Widget::Texture(game.game_target.color()));
        // window.add_widget(Widget::Texture(game.editor.editor_target.color()));
        window.add_widget(Widget::Text(format!("Draw calls: {}", draw_count)));

        if Keyboard::pressed(common::Keycode::E)
            || window.add_widget(Widget::Button("Edit Room", [20, 132, 23, 255]))
        {
            editor.is_showing = true;
        }

        batch.push_matrix(unsafe { GAME_TO_SCREEN_PROJECTION });
        batch.texture(game.game_target.color(), &VEC_2_ZERO);
        batch.pop_matrix();
    }

    // Draw to screen window
    {
        let mouse_position: glm::Vec2 = Mouse::position();
        batch.circle(
            [mouse_position.x, mouse_position.y],
            2.0f32,
            5,
            [255, 255, 255, 255],
        );

        Gui::draw(batch);
        batch.draw_into(&screen_target); // Flush the batch into the screen
        batch.clear();
    }
}
