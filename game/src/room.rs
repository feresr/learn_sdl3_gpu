use std::io::Write;

use common::{utils::texture_atlas::TextureAtlas, IOStream};

pub const TILE_SIZE: usize = 8;
pub const TILES_PER_ROW: usize = 40;
pub const TILES_PER_COLUMN: usize = 24;
pub const TILE_COUNT: usize = (TILES_PER_ROW * TILES_PER_COLUMN) as usize;

// game
// scene (main menu) or layer
// scene (map screen) or layer
// scene (game scene) or layer
// room -> when the user walks to another room, just swap this
// foreground_tiles
// background_tiles
// entities (that should be removed when switching rooms)
// player
// entities (survive room swap, bubble)

// note: room might be doing too much? perhaps add an extra layer
const ROOM_BYTES: &[u8; 1920] =
    include_bytes!("/Users/feresr/Workspace/learn_sdl3_gpu/game/assets/level");

pub struct Room {
    // TODO: pub background_tiles: Tiles,
    pub foreground_tiles: Tiles,
}

impl Room {
    pub fn new() -> Self {
        let mut tiles_array = [Tile::default(); TILE_COUNT];
        unsafe {
            std::ptr::copy_nonoverlapping(
                ROOM_BYTES.as_ptr(),
                tiles_array.as_mut_ptr() as *mut u8,
                ROOM_BYTES.len(),
            );
        }
        Room {
            // background_tiles: Default::default(),
            foreground_tiles: Tiles { inner: tiles_array },
        }
    }

    pub(crate) fn update(&mut self, _atlas: &TextureAtlas) {}

    pub(crate) fn render(&self, batch: &mut common::graphics::batch::Batch, atlas: &TextureAtlas) {
        for (x, y, tile) in &self.foreground_tiles {
            if !tile.visible {
                continue;
            }
            let sprite = atlas.get_index(tile.id.into());
            batch.subtexture(
                sprite,
                glm::vec2(x as f32 * TILE_SIZE as f32, y as f32 * TILE_SIZE as f32),
            );
        }
    }

    pub fn save(&self) {
        let path = "/Users/feresr/Workspace/learn_sdl3_gpu/game/assets/level";
        let mut io = IOStream::from_file(path, "wb").unwrap();
        let bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                self.foreground_tiles.inner.as_ptr() as *const u8,
                std::mem::size_of_val(&self.foreground_tiles.inner),
            )
        };
        io.write(bytes).unwrap();
    }
}

pub struct Tiles {
    inner: [Tile; TILE_COUNT],
}

impl Tiles {
    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        &mut self.inner[x + (TILES_PER_ROW * y)]
    }
}

#[derive(Default, Clone, Copy)]
pub struct Tile {
    pub id: u8,
    pub visible: bool,
}

impl Default for Tiles {
    fn default() -> Self {
        Self {
            inner: [Default::default(); TILE_COUNT],
        }
    }
}

impl<'a> IntoIterator for &'a Tiles {
    // into iterator is so that for loops work
    type Item = (usize, usize, &'a Tile);
    type IntoIter = TileIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TileIterator {
            tiles: &self.inner,
            index: 0,
        }
    }
}

pub struct TileIterator<'a> {
    tiles: &'a [Tile; TILE_COUNT],
    index: usize,
}

impl<'a> Iterator for TileIterator<'a> {
    type Item = (usize, usize, &'a Tile);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= TILE_COUNT {
            return None;
        }

        let current_index = self.index;
        self.index += 1;

        let x = current_index % TILES_PER_ROW as usize;
        let y = current_index / TILES_PER_ROW as usize;
        let tile = &self.tiles[current_index];

        Some((x, y, tile))
    }
}
