use crate::{SCREEN_TO_GAME_PROJECTION, camera::Camera, materials, player::Player, world::World};
use common::{
    Device, Rect, TextureFormat,
    graphics::{
        IDENTITY, batch::Batch, material::Material, render_target::RenderTarget, texture::Texture,
    },
    input::mouse::Mouse,
    ui::gui::Gui,
    utils::tile_atlas::TileAtlas,
};

static ATLAS: &[u8] =
    include_bytes!("/Users/feresr/Workspace/learn_sdl3_gpu/game/assets/atlas.png");

#[allow(dead_code)] // TODO: remove dead code
pub struct Game {
    pub material: Material,
    pub game_target: RenderTarget,
    pub gui: Gui,
    pub world: World,
    pub player: Player,
    pub tile_atlas: TileAtlas,
    pub camera: Camera,
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
            player: Player::new(device.clone()),
            world: World::from_bytes(),
            tile_atlas,
            camera: Camera::default(),
        }
    }

    pub(crate) fn update(&mut self) {
        let game_mouse_position = self.game_mouse_position();
        let window = Gui::window("Game");
        window.set_direction(common::ui::utils::Direction::Vertical);
        window.add_widget(common::ui::widget::Widget::Text(format!(
            "Mouse game position: x:{} y:{}",
            game_mouse_position.x.floor(),
            game_mouse_position.y.floor()
        )));
        window.add_widget(common::ui::widget::Widget::Text(
            "Press 'E' to to enter the map editor.".to_string(),
        ));
        window.add_widget(common::ui::widget::Widget::Text(
            "Press 'R' to hot-reload the game dll.".to_string(),
        ));
        window.add_widget(common::ui::widget::Widget::Text(
            "AWSD to move, SPACE to attack".to_string(),
        ));

        let player_position = self.player.get_position();
        // TODO extract fn to get current room logic into its own funciton (it's being invoked inside Camera too)
        let current_room = self.world.rooms.get_cell_at_position(
            player_position.x as usize,
            (player_position.y + 4) as usize, // TODO 4 is the offset between Romo size and scren size
        );

        self.player.update(&current_room);

        // Follow player (it might have changed room after update(), so we need to re fetch current_room)
        self.camera
            .update(&mut self.game_target, &self.player, &self.world);
    }

    pub(crate) fn render(&self, batch: &mut Batch) {
        // Draw foreground tiles (TODO: Render to an offscreen target only once - composite target)
        let player_position = self.player.get_position();
        let current_room = self.world.rooms.get_cell_at_position(
            player_position.x as usize,
            (player_position.y + 4) as usize, // TODO 4 is the offset between Romo size and scren size
        );

        let game_mouse_position = self.game_mouse_position();
        let mut rect = Rect::new(
            game_mouse_position.x as i32,
            game_mouse_position.y as i32,
            4,
            4,
        );
        rect.offset(-2, -2);
        let collides = current_room.collides(&rect);

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

        current_room.render(batch, &self.tile_atlas);
        self.player.render(batch);
        batch.draw_into(&self.game_target);
    }

    fn game_mouse_position(&self) -> glm::Vec2 {
        Mouse::position_projected(&unsafe { SCREEN_TO_GAME_PROJECTION }) + self.camera.position()
    }
}

// TODO: move this to lib, find better place for this
pub fn create_target_projection(
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
