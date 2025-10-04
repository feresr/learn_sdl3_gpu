use std::io::Write;

use common::{IOStream, Rect, graphics::IDENTITY, utils::tile_atlas::TileAtlas};

use crate::grid::Grid;

pub const TILE_SIZE: usize = 8;
pub const COLUMNS: usize = 40;
pub const ROWS: usize = 24;
pub const TILE_COUNT: usize = (COLUMNS * ROWS) as usize;

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

const WORLD_BYTES: &[u8] =
    include_bytes!("/Users/feresr/Workspace/learn_sdl3_gpu/game/assets/level");

pub const WORLD_COLUMNS: usize = 4;
pub const WORLD_ROWS: usize = 3;
pub const ROOMS_IN_WORLD: usize = WORLD_COLUMNS * WORLD_ROWS;
pub const ROOM_WIDTH: usize = 320;
pub const ROOM_HEIGHT: usize = 180;
pub struct World {
    pub rooms: Grid<Room, ROOMS_IN_WORLD, ROOM_WIDTH, ROOM_HEIGHT, WORLD_COLUMNS, WORLD_ROWS>,
}
impl World {
    pub fn new() -> Self {
        World {
            rooms: Grid {
                inner: unsafe {
                    std::ptr::read(WORLD_BYTES.as_ptr() as *const [Room; ROOMS_IN_WORLD])
                },
            },
        }
    }

    pub(crate) fn render(&self, batch: &mut common::graphics::batch::Batch, atlas: &TileAtlas) {
        for (x, y, room) in &self.rooms {
            batch.push_matrix(glm::translate(
                &IDENTITY,
                &glm::vec3(x as f32 * 320.0, y as f32 * 180.0, 0f32),
            ));
            room.render(batch, atlas);
            batch.pop_matrix()
        }
    }

    pub fn save(&self) {
        let path = "/Users/feresr/Workspace/learn_sdl3_gpu/game/assets/level";
        let mut io = IOStream::from_file(path, "wb").unwrap();
        let bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                self.rooms.inner.as_ptr() as *const u8,
                std::mem::size_of_val(&self.rooms.inner),
            )
        };
        io.write(bytes).unwrap();
    }
}

pub struct Room {
    // TODO: pub background_tiles: Tiles,
    pub foreground_tiles: Grid<Tile, TILE_COUNT, TILE_SIZE, TILE_SIZE, COLUMNS, ROWS>,
}

impl Room {
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Room {
            foreground_tiles: Grid {
                inner: [Tile::default(); TILE_COUNT],
            },
        }
    }

    pub(crate) fn update(&mut self) {}

    pub(crate) fn render(&self, batch: &mut common::graphics::batch::Batch, atlas: &TileAtlas) {
        for (x, y, tile) in &self.foreground_tiles {
            if !tile.visible {
                continue;
            }
            let sprite = atlas.get_at_index(tile.id.into());
            batch.subtexture(
                sprite,
                glm::vec2(x as f32 * TILE_SIZE as f32, y as f32 * TILE_SIZE as f32),
            );
        }
    }

    pub fn collides(&self, rect: &Rect) -> bool {
        let start_x = (rect.left().max(0) as usize / TILE_SIZE).min(COLUMNS);
        let end_x = ((rect.right().max(0) as usize + TILE_SIZE - 1) / TILE_SIZE).min(COLUMNS);
        let start_y = (rect.top().max(0) as usize / TILE_SIZE).min(ROWS);
        let end_y = ((rect.bottom().max(0) as usize + TILE_SIZE - 1) / TILE_SIZE).min(ROWS);

        for y in start_y..end_y.min(ROWS) {
            for x in start_x..end_x.min(COLUMNS) {
                let index = y * COLUMNS + x;
                // Check if the grid cell is occupied
                if self.foreground_tiles[index].visible {
                    let cell_rect = Rect::new(
                        (x * TILE_SIZE) as i32,
                        (y * TILE_SIZE) as i32,
                        TILE_SIZE as u32,
                        TILE_SIZE as u32,
                    );

                    // Check for intersection with the current grid cell
                    if cell_rect.has_intersection(*rect) {
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
