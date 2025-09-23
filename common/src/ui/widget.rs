use crate::{graphics::texture::Texture, ui::window::PADDING};

const AVERAGE_GLYPH_HEIGHT: f32 = 26f32;
pub(crate) const BUTTON_HEIGHT: f32 = 36f32;

pub enum Widget {
    TEXT(&'static str),
    BUTTON(&'static str, [u8;4]),
    TEXTURE(Texture),
    NONE,
}

impl Widget {
    pub(crate) fn cursor_y_offset(&self) -> f32 {
        let offset = match self {
            Widget::TEXT(_) => AVERAGE_GLYPH_HEIGHT, // TODO measure_text? multiple paragraphs
            Widget::BUTTON(_, _) => BUTTON_HEIGHT,
            Widget::TEXTURE(texture) => texture.height as f32,
            Widget::NONE => 0f32,
        };
        offset + PADDING
    }
}

impl Default for Widget {
    fn default() -> Self {
        Widget::NONE
    }
}
