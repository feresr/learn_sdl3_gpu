use common::{Point, Rect, utils::tile_atlas::TileAtlas};

use crate::grid::Grid;

pub const TILE_SIZE: usize = 8;
pub const COLUMNS: usize = 40;
pub const ROWS: usize = 22;
pub const TILE_COUNT: usize = (COLUMNS * ROWS) as usize;

pub const ROOM_WIDTH: usize = 320;
pub const ROOM_HEIGHT: usize = 176;

// game
// scene (main menu) or layer
// scene (map screen) or layer
// scene (game scene) or layer
// room -> when the user walks to another room, just swap this
//      foreground_tiles
//      background_tiles
//      entities (that should be removed when switching rooms)
// player
// entities (that survive room swap, bubble)

pub struct Room {
    // TODO: pub background_tiles: Tiles,
    pub position_in_world: Point,
    pub foreground_tiles: Grid<Tile, TILE_COUNT, TILE_SIZE, TILE_SIZE, COLUMNS, ROWS>,
}

impl Room {
    #[allow(dead_code)]
    pub fn empty(position: Point) -> Self {
        dbg!(position);
        Room {
            position_in_world: position,
            foreground_tiles: Grid {
                inner: [Tile::default(); TILE_COUNT],
            },
        }
    }

    pub(crate) fn render(&self, batch: &mut common::graphics::batch::Batch, atlas: &TileAtlas) {
        let mut tile_position = glm::vec2(0f32, 0f32);
        for (x, y, tile) in &self.foreground_tiles {
            if !tile.visible {
                continue;
            }
            let sprite = atlas.get_at_index(tile.id.into());
            tile_position.x = self.position_in_world.x as f32;
            tile_position.y = self.position_in_world.y as f32;
            tile_position.x += x as f32 * TILE_SIZE as f32;
            tile_position.y += y as f32 * TILE_SIZE as f32;
            batch.subtexture(sprite, tile_position);
        }
    }

    pub fn collides(&self, rect: &Rect) -> bool {
        let mut room_space_rect = rect.clone();
        room_space_rect.offset(
            -self.position_in_world.x as i32,
            -self.position_in_world.y as i32,
        );

        let start_x = (room_space_rect.left().max(0) as usize / TILE_SIZE).min(COLUMNS);
        let end_x =
            ((room_space_rect.right().max(0) as usize + TILE_SIZE - 1) / TILE_SIZE).min(COLUMNS);
        let start_y = (room_space_rect.top().max(0) as usize / TILE_SIZE).min(ROWS);
        let end_y =
            ((room_space_rect.bottom().max(0) as usize + TILE_SIZE - 1) / TILE_SIZE).min(ROWS);

        let mut cell_rect = Rect::new(0, 0, TILE_SIZE as u32, TILE_SIZE as u32);

        for y in start_y..end_y.min(ROWS) {
            for x in start_x..end_x.min(COLUMNS) {
                let index = y * COLUMNS + x;
                // Check if the grid cell is occupied
                if self.foreground_tiles[index].visible {
                    cell_rect
                        .reposition(Point::new((x * TILE_SIZE) as i32, (y * TILE_SIZE) as i32));

                    // Check for intersection with the current grid cell
                    if cell_rect.has_intersection(room_space_rect) {
                        return true; // Collision detected
                    }
                }
            }
        }
        false // No collision detected
    }
}

#[derive(Default, Clone, Copy)]
pub struct Tile {
    pub id: u8,
    pub visible: bool,
}
