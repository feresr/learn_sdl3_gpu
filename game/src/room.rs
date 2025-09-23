pub const TILES_PER_ROW: usize = 40;
pub const TILES_PER_COLUMN: usize = 24;
pub const TILE_COUNT: usize = (TILES_PER_ROW * TILES_PER_COLUMN) as usize;

pub struct Room {
    // TODO: pub background_tiles: Tiles,
    pub foreground_tiles: Tiles,
}

impl Room {
    pub fn new() -> Self {
        let mut foreground: Tiles = Default::default();

        let mut value = 0;
        for tile in &mut foreground.inner {
            tile.id = value;
            value += 2;
            value = value % 3;
        }

        Room {
            // background_tiles: Default::default(),
            foreground_tiles: foreground,
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Tile {
    pub id: u16,
}

pub struct Tiles {
    inner: [Tile; TILE_COUNT],
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
