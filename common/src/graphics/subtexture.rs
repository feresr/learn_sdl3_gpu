use sdl3::{rect::Rect, render::FRect};

use crate::graphics::texture::Texture;

// TODO: Should this be clone? (80 bytes is significative)
// TODO: Make texture field Rc<Texture> (and leave this as Clonable? or also force Rc)
#[derive(Clone)]
pub struct Subtexture {
    pub texture: Texture,
    pub rect: Rect,
    pub uvs: FRect,
}

impl Subtexture {
    pub fn new(texture: Texture, rect: Rect) -> Self {
        let uvs = FRect {
            x: rect.x as f32 / texture.width as f32,
            y: rect.y as f32 / texture.height as f32,
            w: rect.w as f32 / texture.width as f32,
            h: rect.h as f32 / texture.height as f32,
        };

        Self { texture, rect, uvs }
    }
}
