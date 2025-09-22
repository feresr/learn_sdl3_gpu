use crate::{
    graphics::{batch::Batch, texture::Texture},
    input::mouse::Mouse,
    utils::atlas::Atlas,
};

const MAX_WIDGETS: usize = 4;
const HEADER_HEIGHT: f32 = 32f32;
const PADDING: f32 = 8f32;

pub enum Widget {
    TEXT(&'static str),
    BUTTON,
    TEXTURE(Texture),
    NONE,
}

impl Default for Widget {
    fn default() -> Self {
        Widget::NONE
    }
}

#[derive(Default)]
pub struct Window {
    pub position: glm::Vec2,
    pub (crate) size: glm::Vec2,
    pub title: &'static str,
    dragging: bool,
    hovering: bool,
    cursor: glm::Vec2, // Relative to position
    widgets: [Widget; MAX_WIDGETS],
    widget_count: usize,
}

impl Window {
    /**
     * Returns true if this window has (or is about to) captured the drag gesture.
     */
    pub(crate) fn update(&mut self, drag_allowed: bool) -> bool {
        if self.dragging {
            let mouse_rel_position: glm::Vec2 = Mouse::position_rel();
            self.position.x += mouse_rel_position.x;
            self.position.y += mouse_rel_position.y;
        }

        if !Mouse::left_held() {
            self.dragging = false;
        }

        // !drag_allowed means another window is dragging (or about to) do not hover this window
        self.hovering = drag_allowed && self.is_hovering(Mouse::position());
        if Mouse::left_clicked() {
            self.dragging = self.hovering;
        }

        let mut size_x = self.size.x.max(PADDING * 2f32);
        let mut size_y = HEADER_HEIGHT + PADDING * 2f32;
        for widget_index in 0..self.widget_count {
            let widget = &self.widgets[widget_index];
            match widget {
                Widget::TEXT(str) => {
                    const AVERAGE_GLYPH_WIDTH: f32 = 10f32;
                    const AVERAGE_GLYPH_HEIGHT: f32 = 22f32;
                    size_x = size_x.max(str.len() as f32 * AVERAGE_GLYPH_WIDTH);
                    size_y += AVERAGE_GLYPH_HEIGHT;
                }
                Widget::BUTTON => {
                    // TODO
                }
                Widget::TEXTURE(texture) => {
                    size_x = size_x.max(texture.width as f32 + PADDING * 2f32);
                    size_y += texture.height as f32;
                }
                Widget::NONE => {}
            }
        }
        self.size.x = size_x;
        self.size.y = size_y;

        drag_allowed && self.hovering
    }

    pub(crate) fn clear(&mut self) {
        self.widget_count = 0;
        self.cursor.scale_mut(0f32);
    }

    pub(crate) fn draw(&mut self, batch: &mut Batch, atlas: &Atlas) {
        // Draw Background
        const BACKGROUND_COLOR: [u8; 4] = [44, 44, 54, 255];
        batch.rect(
            [self.position.x, self.position.y, 0f32],
            self.size.into(),
            BACKGROUND_COLOR,
        );

        // Draw Header
        const HEADER_COLOR: [u8; 4] = [0, 0, 0, 255];
        const HEADER_COLOR_HOVER: [u8; 4] = [64, 64, 64, 255];
        let header_color = if self.hovering {
            HEADER_COLOR_HOVER
        } else {
            HEADER_COLOR
        };
        batch.rect(
            [self.position.x, self.position.y, 0f32],
            [self.size.x, HEADER_HEIGHT],
            header_color,
        );
        self.cursor.x = PADDING;
        self.cursor.y = 6f32;
        self.draw_text(self.title, batch, atlas);

        // Draw the rest of the widgets
        self.cursor.x = PADDING;
        self.cursor.y = HEADER_HEIGHT + PADDING;
        for widget_index in 0..self.widget_count {
            let widget = &self.widgets[widget_index];
            match widget {
                Widget::TEXT(str) => self.draw_text(str, batch, atlas),
                Widget::BUTTON => {}
                Widget::TEXTURE(texture) => {
                    batch.texture(texture.clone(), self.position + self.cursor);
                    self.cursor += glm::vec2(0f32, texture.height as f32)
                }
                Widget::NONE => {}
            }
        }
    }

    pub fn add_widget(&mut self, widget: Widget) {
        self.widgets[self.widget_count] = widget;
        self.widget_count += 1;
    }

    // TODO: define a Rect interface or similar
    fn is_hovering(&self, mouse_position: glm::Vec2) -> bool {
        mouse_position.x >= self.position.x
            && mouse_position.x <= self.position.x + self.size.x
            && mouse_position.y >= self.position.y
            && mouse_position.y <= self.position.y + HEADER_HEIGHT
    }

    fn draw_text(&mut self, str: &str, batch: &mut Batch, atlas: &Atlas) {
        for ch in str.chars() {
            let (sprite, glyph) = atlas.get_glyph(ch);
            batch.subtexture(
                sprite,
                self.position
                    + self.cursor
                    + glm::vec2(glyph.x_offset as f32, glyph.y_offset as f32),
            );

            self.cursor.x += glyph.x_advance as f32;
        }
    }
}
