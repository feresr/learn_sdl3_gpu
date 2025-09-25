use sdl3::rect::Rect;

use crate::graphics::{subtexture::Subtexture, texture::Texture};

pub struct TextureAtlas {
    texture: Texture,
    tiles_width: u16,
    tiles_height: u16,
    tile_size: u16,
}

impl TextureAtlas {
    pub fn new(texture: Texture, tile_size: u16) -> Self {
        assert!(
            texture.width % tile_size == 0 && texture.height % tile_size == 0,
            "Texture dimensions must be divisible by tile_size"
        );
        let tiles_width = texture.width / tile_size;
        let tiles_height = texture.height / tile_size;
        Self {
            texture,
            tiles_width,
            tiles_height,
            tile_size,
        }
    }

    pub fn get_index(&self, index: usize) -> Subtexture {
        let x = index % self.tiles_width as usize;
        let y = index / self.tiles_width as usize;
        let rect = Rect::new(
            (x * self.tile_size as usize) as i32,
            (y * self.tile_size as usize) as i32,
            self.tile_size as u32,
            self.tile_size as u32,
        );

        Subtexture::new(self.texture.clone(), rect)
    }

    pub fn get(&self, x: u16, y: u16) -> Subtexture {
        let rect = Rect::new(
            (x * self.tile_size) as i32,
            (y * self.tile_size) as i32,
            self.tile_size as u32,
            self.tile_size as u32,
        );

        Subtexture::new(self.texture.clone(), rect)
    }
}

impl<'a> IntoIterator for &'a TextureAtlas {
    type Item = Subtexture;

    type IntoIter = TextureAtlasIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TextureAtlasIterator {
            atlas: &self,
            index: 0,
            tile_count: (self.tiles_height * self.tiles_height) as usize,
        }
    }
}

pub struct TextureAtlasIterator<'a> {
    atlas: &'a TextureAtlas,
    index: usize,
    tile_count: usize,
}

impl<'a> Iterator for TextureAtlasIterator<'a> {
    type Item = Subtexture;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.tile_count {
            return None;
        }
        let current_index = self.index;
        self.index += 1;

        let texture = self.atlas.get_index(current_index);

        Some(texture)
    }
}
