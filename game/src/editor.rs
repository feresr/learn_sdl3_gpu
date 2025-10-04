use common::{
    graphics::IDENTITY,
    input::mouse::Mouse,
    ui::{gui::Gui, utils::Direction, widget::Widget},
    utils::tile_atlas::TileAtlas,
};

use crate::{
    SCREEN_TO_GAME_PROJECTION,
    room::{ROOM_HEIGHT, ROOM_WIDTH, World},
};

pub struct Editor {
    pub showing: bool,
    pub selected_tile: u16,
    pub drawing: bool,
    // Transforms from editor space to game space (where we ultimately render the editor)
    // TODO: GameSpace is not the best name, it's really RenderTargetSpace or PixelGridSpace (which happens to be the same as GameSpace)
    pub editor_to_game_projection: glm::Mat4,
}
/**
 * TODO: Matrix explanation
 * Interpretation A (Incorrect for projection): This describes a change in the coordinate system itself, moving the origin or changing the scale factor of the axes, which isn't the primary function of a projection matrix.
 * Interpretation B (Correct): The matrix tells you where the point should be drawn in the editor's space (space B). If the game world is scaled down, a fixed point in the game world (space A) will map to a smaller coordinate in the editor's screen space (space B), making it appear smaller and closer to the editor's origin.
*/

impl Default for Editor {
    fn default() -> Self {
        Self {
            showing: false,
            selected_tile: 0,
            drawing: false,
            // TODO: editor inputs (pan/zoom)
            editor_to_game_projection: glm::translate(
                &glm::scale(&IDENTITY, &glm::vec3(0.3f32, 0.3f32, 1f32)),
                &glm::vec3(-225f32, -10f32, 0f32),
            ),
        }
    }
}

impl Editor {
    pub fn update(&mut self, world: &mut World, atlas: &TileAtlas) {
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

        window.add_widget(Widget::Text("Selected:".to_string()));
        {
            let mut selected_tile = atlas.get_at_index(self.selected_tile as usize);
            selected_tile.rect.w *= 6;
            selected_tile.rect.h *= 6;
            window.add_widget(Widget::Subtexture(selected_tile));
        }

        window.set_direction(Direction::Horizontal);

        if window.add_widget(Widget::Button("Save room", [20, 182, 23, 255])) {
            world.save();
        }

        if window.add_widget(Widget::Button("Close", [120, 32, 23, 255])) {
            self.showing = false;
        }

        if Mouse::left_clicked() {
            // TODO: select tile and modify properties?
            self.drawing = true;
        }

        // TODO
        if self.drawing {
            if Mouse::left_held() {
                // Moves mouse from screen space -> game space -> editor space
                // Screen space is (0,0) to (os_window_width, os_window_height) it can be resized by the user
                // Game space is (0,0) to (room_width, room_height) or (320,180) per room
                // Editor space is (-inf,-inf) to (inf,inf) - depends on panning and zooming

                let game_to_editor_projection =
                    self.editor_to_game_projection.try_inverse().unwrap();
                let screen_to_editor_projection =
                    game_to_editor_projection * unsafe { SCREEN_TO_GAME_PROJECTION };
                let editor_space_mouse = Mouse::position_projected(&screen_to_editor_projection);

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

                let tile = room
                    .foreground_tiles
                    .get_cell_at_position_mut(room_local_x, room_local_y);
                tile.id = self.selected_tile as u8;
                tile.visible = true;
            } else {
                self.drawing = false;
            }
        }
    }

    pub(crate) fn render(
        &self,
        batch: &mut common::graphics::batch::Batch,
        world: &World,
        atlas: &TileAtlas,
    ) {
        // ZOOM / PADDING
        batch.push_matrix(self.editor_to_game_projection);
        world.render(batch, atlas);
        batch.pop_matrix();
    }
}
