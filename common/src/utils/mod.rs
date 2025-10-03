use std::ops::{Add, Sub};

use crate::graphics::IDENTITY;

pub mod animation;
pub mod font_atlas;
pub mod glyph;
pub mod tile_atlas;

pub fn approach<T>(current: T, value: T, step: T) -> T
where
    T: PartialOrd + Add<Output = T> + Sub<Output = T> + Copy,
{
    if current < value {
        let result = current + step;
        if result > value { value } else { result }
    } else {
        let result = current - step;
        if result < value { value } else { result }
    }
}

pub fn create_transform(position: glm::Vec2, origin: glm::Vec2, scale: glm::Vec2) -> glm::Mat4 {
    return glm::translate(&IDENTITY, &glm::vec3(position.x, position.y, 0.0f32))
        * glm::scale(&IDENTITY, &glm::vec3(scale.x, scale.y, 1.0f32))
        * glm::translate(&IDENTITY, &glm::vec3(-origin.x, -origin.y, 0.0f32));
}
