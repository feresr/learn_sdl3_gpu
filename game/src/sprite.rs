use std::collections::HashMap;

use common::{
    Point, Rect,
    graphics::{VEC_2, subtexture::Subtexture, texture::Texture},
    ui::{gui::Gui, widget::Widget},
    utils::{
        animation::{Animation, Frame},
        create_transform,
    },
};

/**
Sprite
 ├─ Animation (e.g., "idle")
 │   ├─ Frame
 │   ├─ Frame
 ├─ Animation (e.g., "run")
 │   ├─ Frame
 │   ├─ Frame
 └─ Animation (e.g., "attack")
     ├─ Frame
     ├─ Frame
     ├─ Frame
*/

pub struct Sprite {
    frames: Vec<Frame>,
    animations: HashMap<String, Animation>,
    pub frame_index: u8,
    pub looping: bool, // weather the anim has finished its first iteration loop
    pub timer: u32,
    pub playing: Option<Animation>,
    pub scale_x: f32,
    pub scale_y: f32,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl Sprite {
    pub(crate) fn from_atlas(
        texture: Texture, // TODO bake texture name into source (atlas)? To avoid passing two textures here
        source: &str,
    ) -> Self {
        let parser = AtlasParser::texture_and_source(texture, source);
        Sprite {
            frames: parser.frames,
            animations: parser.animations,
            frame_index: 0,
            timer: 0,
            looping: false,
            playing: None,
            scale_x: 1f32,
            scale_y: 1f32,
            flip_x: false,
            flip_y: false,
        }
    }

    pub fn play(&mut self, animation_name: &str) {
        if let Some(current) = &self.playing {
            if current.name == animation_name {
                return;
            }
        }

        let anim: Animation = self.animations.get(animation_name).unwrap().clone();
        self.frame_index = anim.from;
        self.looping = false;
        self.timer = self.frames[self.frame_index as usize].duration;
        self.playing = Some(anim);
    }

    pub(crate) fn update(&mut self) {
        let Some(current_animation) = &self.playing else {
            return;
        };
        if self.timer > 0 {
            self.timer -= 1;
            return;
        }

        // Queue next frame
        self.frame_index += 1;

        // Restart (loop) animation if we've reached the end.
        let animation_frame_count = current_animation.to - current_animation.from;
        if self.frame_index - current_animation.from > animation_frame_count {
            self.frame_index = current_animation.from;
            self.looping = true
        }
        self.timer = self.frames[self.frame_index as usize].duration;
    }

    pub(crate) fn render(&self, position: &Point, batch: &mut common::graphics::batch::Batch) {
        let window = Gui::window("SPRITE");
        window.set_direction(common::ui::utils::Direction::Vertical);

        let Some(current_anim) = &self.playing else {
            return;
        };
        window.add_widget(Widget::Text(format!("PLAYING: {}", current_anim.name)));
        window.add_widget(Widget::Text(format!("frame_index: {}", self.frame_index)));
        window.add_widget(Widget::Text(format!("timer: {}", self.timer)));

        window.add_widget(Widget::Text(format!("Animations:")));
        for (name, anim) in &self.animations {
            window.add_widget(Widget::Text(format!(
                " - {}: {}-{} ",
                name, anim.from, anim.to
            )));
        }

        let frame = &self.frames[self.frame_index as usize];
        let mut subtexture = frame.subtexture.clone();
        subtexture.flip(self.flip_x, self.flip_y);

        let position = glm::vec2(position.x as f32, position.y as f32);
        let origin = glm::vec2(frame.pivot.x as f32, frame.pivot.y as f32);
        let transform = create_transform(position, origin, glm::vec2(self.scale_x, self.scale_y));
        batch.push_matrix(transform);
        batch.subtexture(subtexture, VEC_2);
        batch.pop_matrix();
    }
}

struct AtlasParser {
    frames: Vec<Frame>,
    animations: HashMap<String, Animation>,
}

impl AtlasParser {
    pub fn texture_and_source(texture: Texture, source: &str) -> Self {
        let mut frames: Vec<Frame> = Default::default();
        let mut animations: HashMap<String, Animation> = Default::default();
        for line in source.lines() {
            let mut parts = line.split_ascii_whitespace();
            let first_word = parts.next().expect("Empty line in .atlas file");
            if first_word == "F" {
                // Parse frame
                let x = parts
                    .next()
                    .expect("No x component in .atlas file")
                    .parse()
                    .unwrap();
                let y = parts
                    .next()
                    .expect("No y component in .atlas file")
                    .parse()
                    .unwrap();
                let w = parts
                    .next()
                    .expect("No w component in .atlas file")
                    .parse()
                    .unwrap();
                let h = parts
                    .next()
                    .expect("No h component in .atlas file")
                    .parse()
                    .unwrap();
                let duration = parts
                    .next()
                    .expect("No h component in .atlas file")
                    .parse()
                    .unwrap();
                let pivot_x = parts
                    .next()
                    .expect("No pivot_x component in .atlas file")
                    .parse()
                    .unwrap();
                let pivot_y = parts
                    .next()
                    .expect("No pivot_y component in .atlas file")
                    .parse()
                    .unwrap();

                let rect = Rect::new(x, y, w, h);
                let frame = Frame {
                    subtexture: Subtexture::new(texture.clone(), rect),
                    duration,
                    pivot: Point::new(pivot_x, pivot_y),
                };
                frames.push(frame);
            } else {
                animations.insert(
                    first_word.to_string(),
                    Animation {
                        name: first_word.to_string(),
                        from: parts
                            .next()
                            .expect("Missing from component")
                            .parse()
                            .unwrap(),
                        to: parts
                            .next()
                            .expect("Missing from component")
                            .parse()
                            .unwrap(),
                    },
                );
            }
        }

        Self { frames, animations }
    }
}
