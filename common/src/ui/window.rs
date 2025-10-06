use sdl3::render::FRect;

const DEBUG_LAYOUT: bool = false;

use crate::{
    graphics::batch::Batch,
    input::mouse::Mouse,
    ui::{
        utils::{Direction, MeasuredWidget},
        widget::{BUTTON_HEIGHT, Widget},
    },
    utils::font_atlas::FontAtlas,
};

const MAX_WIDGETS: usize = 26;
pub(crate) const PADDING: f32 = 12f32;
pub(crate) const HEADER_HEIGHT: f32 = 32f32;

#[derive(Default)]
pub struct Window {
    pub position: glm::Vec2,
    pub(crate) size: glm::Vec2,
    pub title: &'static str,
    click: Option<(f32, f32)>,
    dragging: bool,
    hovering_header: bool,
    cursor: glm::Vec2, // Relative to self.position
    // TODO use vec here (with custom allocator)
    widgets: [MeasuredWidget; MAX_WIDGETS],
    widget_count: usize,
    direction: Direction,
}

impl Window {
    /**
     * Returns true if this window has (or is about to) captured the drag gesture.
     * This is invoked at the very start of the frame.
     */
    pub(crate) fn update(&mut self) -> bool {
        if self.dragging {
            let mouse_rel_position: glm::Vec2 = Mouse::position_delta();
            self.position.x += mouse_rel_position.x;
            self.position.y += mouse_rel_position.y;
        }

        if !Mouse::left_held() {
            self.dragging = false;
        }

        self.hovering_header = self.is_hovering_header(Mouse::position());

        let mut focused = false;
        if Mouse::left_clicked() {
            self.dragging = self.hovering_header;
            // Caputure this click (and store it in self.click for future handling)
            if self.is_hovering_window(Mouse::position()) {
                let mouse_position = Mouse::position_relative(self.position);
                // Save this click for future handling (on add_widget(..))
                self.click = Some((mouse_position.x, mouse_position.y));
                // Prevent underlying content (game or another window) from interaacting
                focused = true;
                Mouse::consume_left();
            }
        }
        focused
    }

    pub(crate) fn clear(&mut self) {
        self.widget_count = 0;
        // self.cursor.scale_mut(0f32);

        self.cursor.x = PADDING;
        self.cursor.y = HEADER_HEIGHT + PADDING;
        self.size.x = 0f32;
        self.size.y = 0f32;
    }

    /**
     * Returs true if the user is clicking on this widget
     */
    pub fn add_widget(&mut self, widget: Widget) -> bool {
        // Calculate where this widget should be placed
        // x, y is already provided by the previous widget (cursor)
        let x: f32 = self.cursor.x;
        let y: f32 = self.cursor.y;
        // w, h is defined here
        let w: f32 = widget.width();
        let h: f32 = widget.height();

        // Grow the window to accomodate the new element if needed.
        self.size.x = (self.cursor.x + w + PADDING).max(self.size.x);
        self.size.y = (self.cursor.y + h + PADDING).max(self.size.y);

        // Then calculate where the next widget should be placed (move cursor)
        // take into account direction and this widget size
        match self.direction {
            Direction::Horizontal => {
                self.cursor.x += w + PADDING;
            }
            Direction::Vertical => {
                self.cursor.x = PADDING;
                self.cursor.y += h + PADDING;
            }
        }

        // Check wether this widget was clicked
        let mut clicked = false;
        if let Some((click_x, click_y)) = self.click {
            clicked = click_x > x && click_x < x + w && click_y >= y && click_y <= y + h;
        }

        self.widgets[self.widget_count] = MeasuredWidget {
            widget: widget,
            rect: FRect { x, y, w, h },
        };
        self.widget_count += 1;
        clicked
    }

    pub(crate) fn draw(&mut self, batch: &mut Batch, atlas: &FontAtlas) {
        self.click = None;

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
        let header_color = if self.hovering_header {
            HEADER_COLOR_HOVER
        } else {
            HEADER_COLOR
        };
        batch.rect(
            [self.position.x, self.position.y, 0f32],
            [self.size.x, HEADER_HEIGHT],
            header_color,
        );

        // TODO: Investigate SDL text rendering capabilities instead of custom impl?
        self.draw_text(self.title, glm::vec2(PADDING, 6f32), batch, atlas);

        // Draw the rest of the widgets
        // TODO: move draw into each Widget?
        for widget_index in 0..self.widget_count {
            let widget = &self.widgets[widget_index];
            match &widget.widget {
                Widget::Text(str) => self.draw_text(
                    &str.clone(),
                    glm::vec2(widget.rect.x, widget.rect.y),
                    batch,
                    atlas,
                ),
                Widget::Button(str, color) => {
                    const BUTTON_COLOR_HOVER: [u8; 4] = [14, 14, 14, 255];
                    const BUTTON_COLOR_CLICK: [u8; 4] = [24, 24, 24, 255];
                    // TODO: fix button hovering (it will highlight when overing outside the button - right)
                    let mouse_rel_position = Mouse::position_relative(
                        self.position + glm::vec2(widget.rect.x, widget.rect.y),
                    );
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
                            self.position.x + widget.rect.x,
                            self.position.y + widget.rect.y,
                            0f32,
                        ],
                        // [self.size.x - PADDING * 2f32, BUTTON_HEIGHT],
                        [widget.rect.w, BUTTON_HEIGHT],
                        button_color,
                    );

                    // Draw button label (centered)
                    let label: String = str.to_string();
                    let mut label_position = glm::vec2(widget.rect.x, widget.rect.y);
                    let widget_w = widget.rect.w;
                    let (w, h) = self.measure_text(&label, atlas);
                    label_position.x += (widget_w) / 2f32 - w / 2f32;
                    label_position.y += BUTTON_HEIGHT / 2f32 - h / 2f32;
                    self.draw_text(&label, label_position, batch, atlas);
                }
                Widget::Texture(texture) => {
                    batch.texture(
                        texture.clone(),
                        &(self.position + glm::vec2(widget.rect.x, widget.rect.y)),
                    );
                }
                Widget::Subtexture(subtexture) => {
                    batch.subtexture(
                        subtexture.clone(),
                        self.position + glm::vec2(widget.rect.x, widget.rect.y),
                    );
                }
                Widget::None => {}
            }

            if DEBUG_LAYOUT {
                let widget = &self.widgets[widget_index];
                batch.rect(
                    [
                        self.position.x + widget.rect.x,
                        self.position.y + widget.rect.y,
                        0f32,
                    ],
                    // [self.size.x - PADDING * 2f32, BUTTON_HEIGHT],
                    [widget.rect.w, widget.rect.h],
                    [255, 0, 255, 195],
                );
            }
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    // TODO: define a Rect interface or similar
    fn is_hovering_header(&self, mouse_position: glm::Vec2) -> bool {
        mouse_position.x >= self.position.x
            && mouse_position.x <= self.position.x + self.size.x
            && mouse_position.y >= self.position.y
            && mouse_position.y <= self.position.y + HEADER_HEIGHT
    }
    fn is_hovering_window(&self, mouse_position: glm::Vec2) -> bool {
        mouse_position.x >= self.position.x
            && mouse_position.x <= self.position.x + self.size.x
            && mouse_position.y >= self.position.y
            && mouse_position.y <= self.position.y + self.size.y
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

    fn draw_text(&mut self, str: &str, position: glm::Vec2, batch: &mut Batch, atlas: &FontAtlas) {
        let mut advance = glm::vec2(0f32, 0f32);
        for ch in str.chars() {
            let (sprite, glyph) = atlas.get_glyph(ch);
            batch.subtexture(
                sprite,
                self.position
                    + position
                    + advance
                    + glm::vec2(glyph.x_offset as f32, glyph.y_offset as f32),
            );

            advance.x += glyph.x_advance as f32;
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
