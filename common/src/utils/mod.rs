use std::ops::{Add, Sub};

pub mod font_atlas;
pub mod tile_atlas;
pub mod glyph;
pub mod animation;

pub fn approach<T>(current: T, value: T, step: T) -> T
where
    T: PartialOrd + Add<Output = T> + Sub<Output = T> + Ord + Copy,
{
    if current < value {
        (current + step).clamp(current, value)
    } else {
        (current - step).clamp(value, current)
    }
}