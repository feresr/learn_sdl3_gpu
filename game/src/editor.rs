use common::{
    FPoint,
    graphics::IDENTITY,
    input::{keyboard::Keyboard, mouse::Mouse},
    ui::{gui::Gui, utils::Direction, widget::Widget},
    utils::{create_transform_inplace, tile_atlas::TileAtlas},
};

use crate::{
    room::{ROOM_HEIGHT, ROOM_WIDTH},
    world::World,
};

#[derive(Debug)]
enum Layer {
    Background,
    Foreground,
}

pub struct Editor {
    pub is_showing: bool,
    pub selected_tile: u16,
    offset: FPoint,
    // TODO: impl pinch to zoom (make this the same as aseprite controlls)
    zoom: f32,
    projection: glm::Mat4,
    layer: Layer,
}

impl Editor {
    pub fn default() -> Editor {
        Self {
            is_showing: false,
            selected_tile: 0,
            offset: FPoint::new(0f32, 0f32),
            zoom: 1.0,
            projection: IDENTITY,
            layer: Layer::Foreground,
        }
    }

    pub fn update(&mut self, world: &mut World, atlas: &TileAtlas) {
        self.draw_editor_controls(world, atlas);

        if Keyboard::pressed(common::Keycode::_1) {
            self.apply_zoom_and_pan(1.0);
        }
        if Keyboard::pressed(common::Keycode::_2) {
            self.apply_zoom_and_pan(2.0);
        }
        if Keyboard::pressed(common::Keycode::_3) {
            self.apply_zoom_and_pan(4.0);
        }
        if Keyboard::pressed(common::Keycode::_4) {
            self.apply_zoom_and_pan(8.0);
        }

        let wheel = Mouse::wheel();
        if wheel.y != 0f32 || wheel.x != 0f32 {
            // adjust offset so zoom happens around mouse
            self.offset.x -= wheel.x * 10.0; // Pan left/right with horizontal wheel
            self.offset.y += wheel.y * 10.0; // Pan up/down with vertical wheel
            self.apply_zoom_and_pan(self.zoom);
        }

        if Mouse::left_held() {
            // (x,y) in game space (0,0) to (screen_w, screen_h)
            let screen_space_mouse = Mouse::position();
            // (x,y) in editor space (after zoom and pan)
            let editor_space_mouse = self.projection.try_inverse().unwrap()
                * glm::vec4(screen_space_mouse.x, screen_space_mouse.y, 0f32, 1f32);

            let mouse_x = editor_space_mouse.x as usize;
            let mouse_y = editor_space_mouse.y as usize;

            // Integer division to get the room index in the world grid
            let room_index_x = mouse_x / ROOM_WIDTH;
            let room_index_y = mouse_y / ROOM_HEIGHT;

            let room = world
                .rooms
                .get_cell_at_index_mut(room_index_x, room_index_y);

            // X and Y with origin at this room top-left corner
            let room_local_x = mouse_x - room_index_x * ROOM_WIDTH;
            let room_local_y = mouse_y - room_index_y * ROOM_HEIGHT;

            let tiles = match self.layer {
                Layer::Foreground => &mut room.foreground_tiles,
                Layer::Background => &mut room.background_tiles,
            };
            let tile = tiles.get_cell_at_position_mut(room_local_x, room_local_y);
            tile.id = self.selected_tile as u8;
            tile.visible = true;
        }
    }

    fn apply_zoom_and_pan(&mut self, new_zoom: f32) {
        let screen_space_mouse = Mouse::position();
        let zoom_ratio = new_zoom / self.zoom;
        self.zoom = new_zoom;

        let delta_x = screen_space_mouse.x - self.offset.x;
        let delta_y = screen_space_mouse.y - self.offset.y;
        self.offset.x += delta_x * (1.0 - zoom_ratio);
        self.offset.y += delta_y * (1.0 - zoom_ratio);
        create_transform_inplace(
            &mut self.projection,
            glm::vec2(self.offset.x, self.offset.y),
            glm::vec2(0.0f32, 0.0f32),
            glm::vec2(self.zoom, self.zoom),
        );
    }

    pub(crate) fn render(
        &self,
        batch: &mut common::graphics::batch::Batch,
        world: &World,
        atlas: &TileAtlas,
    ) {
        // Draw transparent square on selected tile (or four 90 angles)
        batch.push_matrix(self.projection);
        world.render(batch, atlas);
        batch.pop_matrix();
    }

    fn draw_editor_controls(&mut self, world: &mut World, atlas: &TileAtlas) {
        let window = Gui::window("Map Editor");
        let mut index = 0;
        for mut tile in atlas {
            window.set_direction(Direction::Horizontal);
            if (index + 1) % 3 == 0 {
                window.set_direction(Direction::Vertical);
            }
            tile.rect.w *= 6;
            tile.rect.h *= 6;
            if window.add_widget(Widget::Subtexture(tile)) {
                self.selected_tile = index;
            }
            index += 1;
        }

        window.set_direction(Direction::Vertical);

        let wheel = Mouse::wheel();
        window.add_widget(Widget::Text(format!("Wheel: {} {} ", wheel.x, wheel.y)));
        window.add_widget(Widget::Text("Selected:".to_string()));
        {
            let mut selected_tile = atlas.get_at_index(self.selected_tile as usize);
            selected_tile.rect.w *= 6;
            selected_tile.rect.h *= 6;
            window.add_widget(Widget::Subtexture(selected_tile));
        }

        window.set_direction(Direction::Horizontal);

        if window.add_widget(Widget::Button("Save world", [20, 182, 23, 255])) {
            world.save();
        }

        if window.add_widget(Widget::Button("Close", [120, 32, 23, 255])) {
            self.is_showing = false;
        }

        window.set_direction(Direction::Vertical);
        if window.add_widget(Widget::Button("Toggle", [0, 0, 32, 255])) {
            match self.layer {
                Layer::Background => self.layer = Layer::Foreground,
                Layer::Foreground => self.layer = Layer::Background,
            }
        }
        window.add_widget(Widget::Text(format!("Drawing in {:?} layer", self.layer)));
    }
}
