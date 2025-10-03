use sdl3::{rect::Rect, render::FRect};

use crate::graphics::texture::Texture;

#[derive(Clone)]
pub struct Subtexture {
    pub texture: Texture,
    pub rect: Rect,
    pub uvs: FRect,
}

impl Subtexture {
    pub fn new(texture: Texture, rect: Rect) -> Self {
        let uvs = FRect {
            x: rect.x as f32 / texture.width() as f32,
            y: rect.y as f32 / texture.height() as f32,
            w: rect.w as f32 / texture.width() as f32,
            h: rect.h as f32 / texture.height() as f32,
        };

        Self { texture, rect, uvs }
    }

    pub fn flip(&mut self, flip_x: bool, flip_y: bool) {
        if flip_x {
            self.uvs.x += self.uvs.w;
            self.uvs.w = -self.uvs.w;
        }
        if flip_y {
            self.uvs.y += self.uvs.h;
            self.uvs.h = -self.uvs.h;
        }
    }
}
