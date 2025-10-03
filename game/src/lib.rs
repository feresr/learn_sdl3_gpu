use common::{
    Device,
    game_memory::GameMemory,
    graphics::{IDENTITY, batch::Batch, render_target::RenderTarget},
    input::{
        keyboard::{KEYBOARD, Keyboard},
        mouse::{MOUSE, Mouse},
    },
    ui::gui::{GUI, Gui},
};

use crate::game::{Game, game_to_screen_projection};

mod editor;
mod game;
mod grid;
mod materials;
mod player;
mod room;
mod sprite;

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
            // game_memory.storage: [[Game] [Arena]]
            // let arena_ptr = game_memory.storage.add(size_of::<Game>()) as *mut Arena<ARENA_SIZE>;
            // arena_ptr.write(Default::default()) ;
            // let arena: &mut Arena<ARENA_SIZE> = &mut *(arena_ptr as *mut Arena<ARENA_SIZE>) ;

            (game_memory.storage as *mut Game).write(Game::new(device.clone()));

            // TODO: clear memory on game exit (no destructors are being invoked here!)
            // Game allocates SDL primitives (Arc) in the heap. Consider sdl3_sys SetMemoryFunctions()
        }
        game_memory.initialized = true;
    }

    let game: &mut Game = unsafe { &mut *(game_memory.storage as *mut Game) };

    unsafe {
        // dbg!((*ARENA_ALLOC.arena).bytes_used());
        // dbg!((*ARENA_ALLOC.arena).capacity());

        KEYBOARD = keyboard as *const Keyboard;
        MOUSE = mouse as *const Mouse;
        GUI = &mut game.gui as *mut Gui;

        if WINDOW_SIZE.0 != screen_target.width || WINDOW_SIZE.1 != screen_target.height {
            GAME_TO_SCREEN_PROJECTION = game_to_screen_projection(&game.game_target, screen_target);
            SCREEN_TO_GAME_PROJECTION = GAME_TO_SCREEN_PROJECTION.try_inverse().unwrap();
            WINDOW_SIZE.0 = screen_target.width;
            WINDOW_SIZE.1 = screen_target.height;
        }
    }

    Gui::update(); // Detect input before anything else
    game.update();

    // Draw to low-res off-screen game target
    {
        game.render(batch);
        batch.draw_into(&game.game_target);
        let draw_count = batch.get_batch_count();

        // Add debug informatoin
        {
            // let window = Gui::window("Offscreen targets");
            // window.set_direction(common::ui::utils::Direction::Vertical);
            // window.add_widget(Widget::Texture(game.game_target.color()));
            // window.add_widget(Widget::Text(format!("Draw calls: {}", draw_count)));
        }

        batch.clear();
    }

    // Draw to screen window
    {
        let mouse_position: glm::Vec2 = Mouse::position();
        batch.push_matrix(unsafe { GAME_TO_SCREEN_PROJECTION });
        batch.texture(game.game_target.color(), glm::vec2(0f32, 0f32));
        batch.pop_matrix();
        batch.circle(
            [mouse_position.x, mouse_position.y],
            2.0f32,
            5,
            [255, 255, 255, 255],
        );

        Gui::draw(batch);
        batch.draw_into(&screen_target);
        batch.clear();
    }

    // unsafe { #[allow(static_mut_refs)]
    //     A.celar_arena() };

    // TODO (Important!) we need to clear 'Game' properly when exiting the frame loop
    // It contains Rc to objects stored in the heap (Mostly SDL resources)

    // TODO: game_state.arena.reset();
}
