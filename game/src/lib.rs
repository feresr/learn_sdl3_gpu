use std::mem;

use common::{
    Device, TextureFormat,
    game_memory::GameMemory,
    graphics::{
        IDENTITY, batch::Batch, material::Material, render_target::RenderTarget, texture::Texture,
    },
};
mod materials;

extern crate nalgebra_glm as glm;

struct GameState {
    material: Material,
    game_target: RenderTarget,
    game_to_screen_projection: glm::Mat4,
    dummy_texture: Texture,
}

static FOO: &[u8; 488] =
    include_bytes!("/Users/feresr/Workspace/learn_sdl3_gpu/game/assets/atlas-normal.png");

impl GameState {
    fn new(screen_target: &RenderTarget, device: Device) -> Self {
        let offscreen_target = RenderTarget::new(Texture::new(
            device.clone(),
            320,
            180,
            TextureFormat::R8g8b8a8Unorm,
        ));
        let game_to_screen_projection =
            create_game_to_screen_target(&screen_target, &offscreen_target);

        let dummy_texture = Texture::from_bytes(device.clone(), FOO);
        GameState {
            material: Material::from_specification(device.clone(), &materials::RED_MATERIAL),
            game_target: offscreen_target,
            game_to_screen_projection,
            dummy_texture,
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn update_game(
    game_memory: *mut GameMemory,
    batch: &mut Batch,
    screen_target: &mut RenderTarget,
    device: &Device,
) {
    let memory: &mut GameMemory = unsafe { &mut *game_memory };
    if !memory.initialized {
        debug_assert!(
            std::mem::size_of::<GameState>() <= GameMemory::GAME_MEMORY,
            "GameState ({}) is larger than the available game memory ({})",
            std::mem::size_of::<GameState>(),
            GameMemory::GAME_MEMORY
        );
        unsafe {
            (memory.storage as *mut GameState).write(GameState::new(&screen_target, device.clone()))
        }
        memory.initialized = true;
    }

    let game_state: &mut GameState = unsafe { &mut *(memory.storage as *mut GameState) };

    // Draw to low-res off-screen game target
    {
        batch.push_material(&game_state.material);
        batch.circle([25.0f32, 90.0f32], 14.0f32, 54, [255, 255, 255, 255]);
        batch.pop_material();

        batch.texture(game_state.dummy_texture.clone(), glm::vec2(0f32, 0f32));
        batch.draw_into(&game_state.game_target);
    }

    // Draw to screen window
    {
        batch.push_matrix(game_state.game_to_screen_projection);
        batch.texture(game_state.game_target.color(), glm::vec2(0f32, 0f32));
        batch.pop_matrix();

        batch.draw_into(&screen_target);
        batch.clear();
    }
}

// TODO: find a better place for this
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
