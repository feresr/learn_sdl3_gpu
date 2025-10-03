use crate::{SCREEN_TO_GAME_PROJECTION, editor::Editor, materials, player::Player, room::Room};
use common::{
    Device, Rect, TextureFormat,
    graphics::{
        IDENTITY, batch::Batch, material::Material, render_target::RenderTarget, texture::Texture,
    },
    input::mouse::Mouse,
    ui::{gui::Gui, widget::Widget},
    utils::tile_atlas::TileAtlas,
};

static ATLAS: &[u8] =
    include_bytes!("/Users/feresr/Workspace/learn_sdl3_gpu/game/assets/atlas.png");

pub struct Game {
    pub material: Material,
    pub game_target: RenderTarget,
    pub gui: Gui,
    pub room: Room,
    pub player: Player,
    pub editor: Editor,
    pub tile_atlas: TileAtlas,
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
        let tile_atlas = TileAtlas::new(atlas_texture, 8);

        Game {
            material: Material::from_specification(device.clone(), &materials::RED_MATERIAL),
            game_target: offscreen_target,
            gui: Gui::new(device.clone()),
            // arena: Default::default(),
            editor: Default::default(),
            player: Player::new(device.clone()),
            room: Room::new(),
            tile_atlas,
        }
    }

    pub(crate) fn update(&mut self) {
        let game_mouse_position = Mouse::position_projected(&unsafe { SCREEN_TO_GAME_PROJECTION });

        let window = Gui::window("Game");
        window.add_widget(common::ui::widget::Widget::Text(format!(
            "Mouse game position: x:{} y:{}",
            game_mouse_position.x.floor(),
            game_mouse_position.y.floor()
        )));
        window.add_widget(common::ui::widget::Widget::Text(
            "Press 'R' to hot-reload the game dll.".to_string(),
        ));
        window.add_widget(common::ui::widget::Widget::Text(
            "AWSD to move, SPACE to attack".to_string(),
        ));

        if self.editor.showing {
            self.editor.update(&mut self.room, &self.tile_atlas);
            return;
        }

        self.room.update();
        self.player.update(&self.room);

        if window.add_widget(Widget::Button("Edit Room", [20, 132, 23, 255])) {
            self.editor.showing = true;
        }
    }

    pub(crate) fn render(&self, batch: &mut Batch) {
        // Draw foreground tiles (TODO: Render to an offscreen target only once - composite target)
        if self.editor.showing {
            self.editor.render(batch, &self.room, &self.tile_atlas);
            return;
        }
        self.room.render(batch, &self.tile_atlas);

        let game_mouse_position = &Mouse::position_projected(&unsafe { SCREEN_TO_GAME_PROJECTION });

        let mut rect = Rect::new(
            game_mouse_position.x as i32,
            game_mouse_position.y as i32,
            4,
            4,
        );
        rect.offset(-2, -2);
        let collides = self.room.collides(&rect);

        if collides {
            batch.rect(
                [rect.x as f32, rect.y as f32, 0f32],
                [4f32, 4f32],
                [0, 255, 0, 255],
            );
        } else {
            batch.rect(
                [rect.x as f32, rect.y as f32, 0f32],
                [4f32, 4f32],
                [255, 255, 255, 255],
            );
        }

        self.player.render(batch);
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
