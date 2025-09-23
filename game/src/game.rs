use crate::{SCREEN_TO_GAME_PROJECTION, materials, room::Room};
use common::{
    Device, TextureFormat,
    graphics::{
        IDENTITY, batch::Batch, material::Material, render_target::RenderTarget, texture::Texture,
    },
    input::mouse::Mouse,
    ui::Gui,
    utils::texture_atlas::TextureAtlas,
};

static ATLAS: &[u8; 402] =
    include_bytes!("/Users/feresr/Workspace/learn_sdl3_gpu/game/assets/atlas.png");

pub struct Game {
    pub material: Material,
    pub game_target: RenderTarget,
    pub dummy_bool: bool,
    pub gui: Gui,
    // pub arena: Arena<128>, TODO: needed?
    pub room: Room,
    pub atlas: TextureAtlas,
}

impl Game {
    pub fn new(device: Device) -> Self {
        let offscreen_target = RenderTarget::new(Texture::new(
            device.clone(),
            320,
            180,
            TextureFormat::R8g8b8a8Unorm,
        ));

        let atlas_texture = Texture::from_bytes(device.clone(), ATLAS);
        let atlas = TextureAtlas::new(atlas_texture, 8);

        Game {
            material: Material::from_specification(device.clone(), &materials::RED_MATERIAL),
            game_target: offscreen_target,
            dummy_bool: false,
            gui: Gui::new(device.clone()),
            // arena: Default::default(),
            room: Room::new(),
            atlas,
        }
    }

    pub(crate) fn update(&mut self) {}

    pub(crate) fn render(&self, batch: &mut Batch) {
        let game_mouse_position = Mouse::position_projected(&unsafe { SCREEN_TO_GAME_PROJECTION });

        // Draw foreground tiles (TODO: Render to an offscreen target only once - composite target)
        self.room.render(batch, &self.atlas);

        Gui::window("Game mouse position").add_widget(common::ui::widget::Widget::TEXT(format!(
            "{} {}",
            game_mouse_position.x, game_mouse_position.y
        )));

        batch.push_material(&self.material);
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
    }
}

pub fn game_to_screen_projection(
    game_target: &RenderTarget,
    screen_target: &RenderTarget,
) -> glm::Mat4 {
    let scale = (screen_target.width as f32 / game_target.width as f32)
        .min(screen_target.height as f32 / game_target.height as f32);

    let screen_center: glm::Vec2 = glm::vec2(
        screen_target.width as f32 / 2f32,
        screen_target.height as f32 / 2f32,
    );
    let game_center: glm::Vec2 = glm::vec2(
        game_target.width as f32 / 2f32,
        game_target.height as f32 / 2f32,
    );

    return create_transform(screen_center, game_center, glm::vec2(scale, scale));
}

// TODO: Dead code
#[allow(dead_code)]
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
