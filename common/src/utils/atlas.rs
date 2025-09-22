use sdl3::rect::Rect;

use crate::{
    graphics::{subtexture::Subtexture, texture::Texture},
    utils::glyph::{Glyph, GlyphData},
};

pub struct Atlas {
    texture: Texture,
    glyph_data: GlyphData,
}

impl Atlas {
    pub fn new(texture: Texture, glyph_data : GlyphData) -> Self {
        Atlas { texture, glyph_data }
    }

    pub fn get_glyph(&self, glyph: char) -> (Subtexture, &Glyph) {
        let glyph = self.glyph_data.get(glyph);
        let sprite = Subtexture::new(
            self.texture.clone(),
            Rect::new(
                glyph.x.into(),
                glyph.y.into(),
                glyph.width.into(),
                glyph.height.into(),
            ),
        );

        (sprite, glyph)
    }
}
