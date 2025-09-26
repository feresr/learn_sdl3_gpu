use sdl3::render::FRect;

use crate::ui::widget::Widget;


pub(crate) struct MeasuredWidget {
    pub(crate) widget: Widget,
    pub(crate) rect: FRect,
}

impl Default for MeasuredWidget {
    fn default() -> Self {
        Self {
            widget: Default::default(),
            rect: FRect::new(0f32, 0f32, 0f32, 0f32),
        }
    }
}
pub enum Direction {
    // TODO move this to wher its used
    Horizontal,
    Vertical,
}
impl Default for Direction {
    fn default() -> Self {
        Direction::Vertical
    }
}