use sdl3::rect::Point;

use crate::graphics::subtexture::Subtexture;

#[derive(Clone)]
pub struct Animation {
    pub from: u8,
    pub to: u8,
    pub name: String,
}

#[derive(Clone)]
pub struct Frame {
    pub subtexture: Subtexture,
    pub duration: u32,
    pub pivot: Point,
}