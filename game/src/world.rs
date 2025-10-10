use std::io::Write;

use common::{IOStream, Point, utils::tile_atlas::TileAtlas};

use crate::{
    grid::Grid,
    room::{ROOM_HEIGHT, ROOM_WIDTH, Room},
};

const LEVEL_PATH: &str = "/Users/feresr/Workspace/learn_sdl3_gpu/game/assets/level";
const WORLD_BYTES: &[u8] =
    include_bytes!("/Users/feresr/Workspace/learn_sdl3_gpu/game/assets/level");

pub const WORLD_COLUMNS: usize = 4;
pub const WORLD_ROWS: usize = 3;
pub const ROOMS_IN_WORLD: usize = WORLD_COLUMNS * WORLD_ROWS;
pub struct World {
    pub rooms: Grid<Room, ROOMS_IN_WORLD, ROOM_WIDTH, ROOM_HEIGHT, WORLD_COLUMNS, WORLD_ROWS>,
}
impl World {
    pub fn from_bytes() -> Self {
        World {
            rooms: Grid {
                inner: unsafe {
                    std::ptr::read(WORLD_BYTES.as_ptr() as *const [Room; ROOMS_IN_WORLD])
                },
            },
        }
    }

    pub fn blank() -> Self {
        World {
            rooms: Grid {
                inner: core::array::from_fn(|i| {
                    Room::empty(Point::new(
                        (i as i32 % WORLD_COLUMNS as i32) * 320,
                        ((i as i32 / WORLD_COLUMNS as i32) * 176i32) as i32,
                    ))
                }),
            },
        }
    }

    pub(crate) fn render(&self, batch: &mut common::graphics::batch::Batch, atlas: &TileAtlas) {
        // TODO: rooms have their own position don't need x y here
        for (_, _, room) in &self.rooms {
            room.render(batch, atlas);
            batch.rect_outline(
                [
                    room.position_in_world.x as f32,
                    room.position_in_world.y as f32,
                    0f32,
                ],
                [ROOM_WIDTH as f32, ROOM_HEIGHT as f32],
                [255, 255, 255, 255],
                1.0f32,
            );
        }
    }

    pub fn save(&self) {
        let mut io = IOStream::from_file(LEVEL_PATH, "wb").unwrap();
        let bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                self.rooms.inner.as_ptr() as *const u8,
                std::mem::size_of_val(&self.rooms.inner),
            )
        };
        io.write(bytes).unwrap();
    }
}
