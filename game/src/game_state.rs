use crate::materials;
use common::{
    Device, Rect, TextureFormat,
    graphics::{
        IDENTITY, material::Material, render_target::RenderTarget, subtexture::Subtexture,
        texture::Texture,
    },
    ui::Gui,
};

static FOO: &[u8; 402] =
    include_bytes!("/Users/feresr/Workspace/learn_sdl3_gpu/game/assets/atlas.png");

pub struct GameState {
    pub material: Material,
    pub game_target: RenderTarget,
    pub dummy_texture: Texture,
    pub dummy_subtexture: Subtexture,
    pub dummy_position: glm::Vec2,
    pub dummy_string: String,
    pub gui: Gui,
}

impl GameState {
    pub fn new(device: Device) -> Self {
        let offscreen_target = RenderTarget::new(Texture::new(
            device.clone(),
            320,
            180,
            TextureFormat::R8g8b8a8Unorm,
        ));

        let dummy_texture = Texture::from_bytes(device.clone(), FOO);
        let dummy_subtexture = Subtexture::new(dummy_texture.clone(), Rect::new(8, 8, 8, 8));
        GameState {
            material: Material::from_specification(device.clone(), &materials::RED_MATERIAL),
            game_target: offscreen_target,
            dummy_texture,
            dummy_subtexture,
            dummy_position: glm::Vec2::default(),
            dummy_string: Default::default(),
            gui: Gui::new(device.clone()),
        }
    }
}

// TODO: Optimize, avoid re-creating on every frame
pub fn game_to_screen_projection(
    game_target: &RenderTarget,
    screen_target: &RenderTarget,
) -> glm::Mat4 {
    return create_game_to_screen_target_projection(screen_target, game_target);
}

// TODO: find a better place for this
pub fn create_game_to_screen_target_projection(
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

    return create_transform(screen_center, game_center, glm::vec2(scale, scale));
}

pub fn apply_transform_inplace(
    mut mat: glm::Mat4,
    position: glm::Vec2,
    origin: glm::Vec2,
    scale: glm::Vec2,
) -> glm::Mat4 {
    mat.fill_with_identity();
    mat.append_translation_mut(&glm::vec3(-origin.x, -origin.y, 0.0f32));
    mat.append_nonuniform_scaling_mut(&glm::vec3(scale.x, scale.y, 1.0f32));
    mat.append_translation_mut(&glm::vec3(position.x, position.y, 0.0f32));
    mat
}

// TODO: Find a better place for this
pub fn create_transform(position: glm::Vec2, origin: glm::Vec2, scale: glm::Vec2) -> glm::Mat4 {
    return glm::translate(&IDENTITY, &glm::vec3(position.x, position.y, 0.0f32))
        * glm::scale(&IDENTITY, &glm::vec3(scale.x, scale.y, 1.0f32))
        * glm::translate(&IDENTITY, &glm::vec3(-origin.x, -origin.y, 0.0f32));
}
