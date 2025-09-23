use sdl3::rect::Rect;

use crate::graphics::{subtexture::Subtexture, texture::Texture};

pub struct TextureAtlas {
    texture: Texture,
    tile_size: u16,
}

impl TextureAtlas {
    pub fn new(texture: Texture, tile_size: u16) -> Self {
        Self { texture, tile_size }
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
