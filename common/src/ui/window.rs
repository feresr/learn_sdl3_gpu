use crate::{
    graphics::{batch::Batch, texture::Texture},
    input::mouse::Mouse,
};

const MAX_WIDGETS: usize = 4;
pub enum Widget {
    TEXT,
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
    pub size: glm::Vec2,
    dragging: bool,
    cursor: glm::Vec2, // Relative to position
    widgets: [Widget; MAX_WIDGETS],
    widget_count: usize,
}

impl Window {
    pub(crate) fn update(&mut self) {
        if self.dragging {
            let mouse_rel_position: glm::Vec2 = Mouse::position_rel();
            self.position.x += mouse_rel_position.x;
            self.position.y += mouse_rel_position.y;
        }

        if !Mouse::left_held() {
            self.dragging = false;
        }
        if Mouse::left_clicked() {
            let mouse_position: glm::Vec2 = Mouse::position();
            self.dragging = self.is_hovering(mouse_position);
        }
    }

    pub(crate) fn clear(&mut self) {
        self.widget_count = 0;
        self.cursor.scale_mut(0f32);
    }

    pub(crate) fn draw(&mut self, batch: &mut Batch) {
        let color;
        if self.dragging {
            color = [250, 4, 102, 255];
        } else {
            color = [50, 4, 132, 255];
        }

        // Background
        batch.rect(
            [self.position.x, self.position.y, 0f32],
            self.size.into(),
            color,
        );

        // Header
        const HEADER_HEIGHT: f32 = 28f32;
        batch.rect(
            [self.position.x, self.position.y, 0f32],
            [self.size.x, HEADER_HEIGHT],
            [0, 0, 0, 255],
        );
        self.cursor.y = HEADER_HEIGHT;

        for widget in &self.widgets {
            match widget {
                Widget::TEXT => {}
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
    pub fn is_hovering(&self, mouse_position: glm::Vec2) -> bool {
        mouse_position.x >= self.position.x
            && mouse_position.x <= self.position.x + self.size.x
            && mouse_position.y >= self.position.y
            && mouse_position.y <= self.position.y + self.size.y
    }
}
