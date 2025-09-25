use common::{
    input::mouse::Mouse,
    ui::{Gui, widget::Widget},
    utils::texture_atlas::TextureAtlas,
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
    pub fn update(&mut self, room: &mut Room, atlas: &TextureAtlas) {
        let window = Gui::window("Map Editor");

        let mut index = 0;
        for mut tile in atlas {
            tile.rect.w *= 6;
            tile.rect.h *= 6;
            if window.add_widget(Widget::SPRITE(tile)) {
                self.selected_tile = index;
            }
            index += 1;
        }

        if window.add_widget(Widget::BUTTON("Close", [120, 32, 23, 255])) {
            self.showing = false;
        }

        let mouse_position = &Mouse::position_projected(&unsafe { SCREEN_TO_GAME_PROJECTION });
        if Mouse::left_clicked() {
            self.drawing = true;
        }

        if self.drawing {
            if Mouse::left_held() {
                room.foreground_tiles
                    .get_tile_mut(
                        (mouse_position.x / TILE_SIZE as f32) as usize,
                        (mouse_position.y / TILE_SIZE as f32) as usize,
                    )
                    .id = self.selected_tile;
            } else {
                self.drawing = false;
            }
        }
    }

    pub(crate) fn render(
        &self,
        batch: &mut common::graphics::batch::Batch,
        room: &Room,
        atlas: &TextureAtlas,
    ) {
        room.render(batch, atlas);
    }
}
