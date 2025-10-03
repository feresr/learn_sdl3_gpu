use common::{
    input::mouse::Mouse,
    ui::{gui::Gui, utils::Direction, widget::Widget},
    utils::tile_atlas::TileAtlas,
};

use crate::{
    SCREEN_TO_GAME_PROJECTION,
    room::{Room, TILE_SIZE},
};

#[derive(Default)]
pub struct Editor {
    pub showing: bool,
    pub selected_tile: u16,
    pub drawing: bool,
}

impl Editor {
    pub fn update(&mut self, room: &mut Room, atlas: &TileAtlas) {
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
            room.save();
        }

        if window.add_widget(Widget::Button("Close", [120, 32, 23, 255])) {
            self.showing = false;
        }

        let mouse_position = &Mouse::position_projected(&unsafe { SCREEN_TO_GAME_PROJECTION });
        if Mouse::left_clicked() {
            self.drawing = true;
        }

        if self.drawing {
            if Mouse::left_held() {
                let tile = room.foreground_tiles.get_tile_mut(
                    (mouse_position.x / TILE_SIZE as f32) as usize,
                    (mouse_position.y / TILE_SIZE as f32) as usize,
                );

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
        room: &Room,
        atlas: &TileAtlas,
    ) {
        // IDea: select tile and modify properties?
        // For panning, just push a metrix here
        room.render(batch, atlas);
    }
}
