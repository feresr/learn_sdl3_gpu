use std::io::Write;

use common::{IOStream, graphics::IDENTITY, utils::tile_atlas::TileAtlas};

use crate::{
    grid::Grid,
    room::{ROOM_HEIGHT, ROOM_WIDTH, Room},
};

const WORLD_BYTES: &[u8] =
    include_bytes!("/Users/feresr/Workspace/learn_sdl3_gpu/game/assets/level");

pub const WORLD_COLUMNS: usize = 4;
pub const WORLD_ROWS: usize = 3;
pub const ROOMS_IN_WORLD: usize = WORLD_COLUMNS * WORLD_ROWS;
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

    pub fn blank() -> Self {
        World {
            rooms: Grid {
                inner: core::array::from_fn(|_| Room::empty()),
            },
        }
    }

    pub(crate) fn render(&self, batch: &mut common::graphics::batch::Batch, atlas: &TileAtlas) {
        for (x, y, room) in &self.rooms {
            batch.push_matrix(glm::translate(
                &IDENTITY,
                &glm::vec3(
                    x as f32 * (ROOM_WIDTH as f32),
                    y as f32 * ROOM_HEIGHT as f32,
                    0f32,
                ),
            ));
            room.render(batch, atlas);
            batch.rect_outline(
                [0f32, 0f32, 0f32],
                [ROOM_WIDTH as f32, ROOM_HEIGHT as f32],
                [255, 255, 255, 255],
                1.0f32,
            );
            // batch.rect(position, size, color);
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
