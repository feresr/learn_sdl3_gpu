use crate::{
    graphics::batch::Batch,
    input::mouse::Mouse,
    ui::widget::{BUTTON_HEIGHT, Widget},
    utils::font_atlas::FontAtlas,
};

const MAX_WIDGETS: usize = 6;
pub(crate) const PADDING: f32 = 8f32;
pub(crate) const HEADER_HEIGHT: f32 = 32f32;

#[derive(Default)]
pub struct Window {
    pub position: glm::Vec2,
    pub(crate) size: glm::Vec2,
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
    pub(crate) fn update(&mut self, drag_allowed: bool, atlas: &FontAtlas) -> bool {
        if self.dragging {
            let mouse_rel_position: glm::Vec2 = Mouse::position_delta();
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
        let mut size_y = HEADER_HEIGHT + PADDING;
        for widget_index in 0..self.widget_count {
            let widget = &self.widgets[widget_index];
            size_y += widget.cursor_y_offset();
            match widget {
                Widget::TEXT(str) => {
                    let str: String = str.to_string();
                    let (w, _) = self.measure_text(&str, atlas);
                    size_x = size_x.max(w + PADDING * 2f32);
                }
                Widget::TEXTURE(texture) => {
                    size_x = size_x.max(texture.width as f32 + PADDING * 2f32);
                }
                Widget::BUTTON(str, _) => {
                    let str: String = str.to_string();
                    let (w, _) = self.measure_text(&str, atlas);
                    size_x = size_x.max(w + PADDING * 2f32);
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

    pub(crate) fn draw(&mut self, batch: &mut Batch, atlas: &FontAtlas) {
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
        // TODO: move draw into each Widget?
        self.cursor.x = PADDING;
        self.cursor.y = HEADER_HEIGHT + PADDING;
        for widget_index in 0..self.widget_count {
            let widget = &self.widgets[widget_index];
            let y_offset = widget.cursor_y_offset();
            match widget {
                Widget::TEXT(str) => self.draw_text(&str.clone(), batch, atlas),
                Widget::BUTTON(str, color) => {
                    const BUTTON_COLOR_HOVER: [u8; 4] = [14, 14, 14, 255];
                    const BUTTON_COLOR_CLICK: [u8; 4] = [24, 24, 24, 255];
                    let mouse_rel_position = Mouse::position_relative(self.position + self.cursor);
                    let button_color = if mouse_rel_position.x >= 0f32
                        && mouse_rel_position.x <= self.size.x - PADDING * 2f32
                        && mouse_rel_position.y >= 0f32
                        && mouse_rel_position.y <= BUTTON_HEIGHT
                    {
                        if Mouse::left_held() {
                            Self::add_arrays(color, &BUTTON_COLOR_CLICK)
                        } else {
                            Self::add_arrays(color, &BUTTON_COLOR_HOVER)
                        }
                    } else {
                        *color
                    };

                    // Draw button background
                    batch.rect(
                        [
                            self.position.x + self.cursor.x,
                            self.position.y + self.cursor.y,
                            0f32,
                        ],
                        [self.size.x - PADDING * 2f32, BUTTON_HEIGHT],
                        button_color,
                    );

                    // Draw button label (centered)
                    let label: String = str.to_string();
                    let (w, h) = self.measure_text(&label, atlas);
                    self.cursor.x += (self.size.x - PADDING) / 2f32 - w / 2f32;
                    self.cursor.y += BUTTON_HEIGHT / 2f32 - h / 2f32;
                    self.draw_text(&label, batch, atlas);
                    // Revert cursor to its orignal position
                    self.cursor.x -= (self.size.x - PADDING) / 2f32 - w / 2f32;
                    self.cursor.y -= BUTTON_HEIGHT / 2f32 - h / 2f32;
                }
                Widget::TEXTURE(texture) => {
                    batch.texture(texture.clone(), self.position + self.cursor);
                }
                Widget::NONE => {}
            }

            self.cursor.x = PADDING;
            self.cursor.y += y_offset;
        }
    }

    /**
     * The returned boolean is interpreted within context:
     * Widget::Button -> Weather the button was clicked
     */
    pub fn add_widget(&mut self, widget: Widget) -> bool {
        let mut clicked = false;
        if Mouse::left_clicked() {
            match widget {
                Widget::BUTTON(_, _) => {
                    // Calculate button position and detect hover/click
                    let mut cursor_height = HEADER_HEIGHT + PADDING;
                    for widget_index in 0..self.widget_count {
                        let widget = &self.widgets[widget_index];
                        cursor_height += widget.cursor_y_offset();
                    }
                    let mouse_position = Mouse::position_relative(self.position);
                    clicked = mouse_position.x > 0f32
                        && mouse_position.x < self.size.y
                        && mouse_position.y >= cursor_height
                        && mouse_position.y <= cursor_height + BUTTON_HEIGHT;
                }
                _ => (),
            }
        }
        self.widgets[self.widget_count] = widget;
        self.widget_count += 1;
        clicked
    }

    // TODO: define a Rect interface or similar
    fn is_hovering(&self, mouse_position: glm::Vec2) -> bool {
        mouse_position.x >= self.position.x
            && mouse_position.x <= self.position.x + self.size.x
            && mouse_position.y >= self.position.y
            && mouse_position.y <= self.position.y + HEADER_HEIGHT
    }

    fn measure_text(&mut self, str: &str, atlas: &FontAtlas) -> (f32, f32) {
        let start_cursor_x_position = self.cursor.x;
        let mut glyph_height = 0f32;
        let mut end_cursor_x_position = start_cursor_x_position;
        for ch in str.chars() {
            let (_, glyph) = atlas.get_glyph(ch);
            end_cursor_x_position += glyph.x_advance as f32;
            glyph_height = glyph_height.max((glyph.height as i16 + glyph.y_offset) as f32);
        }

        (
            end_cursor_x_position - start_cursor_x_position,
            glyph_height,
        )
    }

    fn draw_text(&mut self, str: &str, batch: &mut Batch, atlas: &FontAtlas) {
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

    fn add_arrays(a: &[u8; 4], b: &[u8; 4]) -> [u8; 4] {
        let mut result = [0u8; 4];
        for i in 0..4 {
            result[i] = a[i].saturating_add(b[i]);
        }
        result
    }
}
