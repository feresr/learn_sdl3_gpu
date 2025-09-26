use crate::graphics::{subtexture::Subtexture, texture::Texture};

const AVERAGE_GLYPH_HEIGHT: f32 = 24f32;
const AVERAGE_GLYPH_WIDTH: f32 = 8f32;
pub(crate) const BUTTON_HEIGHT: f32 = 36f32;

pub enum Widget {
    Text(String),
    Button(&'static str, [u8; 4]),
    Texture(Texture),
    Subtexture(Subtexture),
    // TODO: CheckBox
    None,
}

impl Widget {
    pub(crate) fn height(&self) -> f32 {
        let offset = match self {
            Widget::Text(_) => AVERAGE_GLYPH_HEIGHT,
            Widget::Button(_, _) => BUTTON_HEIGHT,
            Widget::Texture(texture) => texture.height as f32,
            Widget::Subtexture(subtexture) => subtexture.rect.height() as f32,
            Widget::None => 0f32,
        };
        offset
    }
    pub(crate) fn width(&self) -> f32 {
        let offset = match self {
            Widget::Text(str) => str.len() as f32 * AVERAGE_GLYPH_WIDTH, // todo
            Widget::Button(_, _) => 180f32, // TODO: allow user to pass custom width
            Widget::Texture(texture) => texture.width as f32,
            Widget::Subtexture(subtexture) => subtexture.rect.width() as f32,
            Widget::None => 0f32,
        };
        offset
    }
}

impl Default for Widget {
    fn default() -> Self {
        Widget::None
    }
}
