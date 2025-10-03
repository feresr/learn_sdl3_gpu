use common::{
    Device, Point, Rect,
    graphics::{batch::Batch, texture::Texture},
    input::keyboard::Keyboard,
};

use crate::{room::Room, sprite::Sprite};

struct Mover {
    speed: glm::Vec2,
    reminder: glm::Vec2,
}

impl Default for Mover {
    fn default() -> Self {
        Self {
            speed: glm::vec2(1f32, 1f32),
            reminder: Default::default(),
        }
    }
}

impl Mover {
    pub fn get_integer_move_amount(&mut self) -> Point {
        let total_speed: glm::Vec2 = self.reminder + self.speed;
        self.speed.x = total_speed.x as i32 as f32;
        self.speed.y = total_speed.y as i32 as f32;
        self.reminder.x = total_speed.x - self.speed.x;
        self.reminder.y = total_speed.y - self.speed.y;

        Point::new(self.speed.x as i32, self.speed.y as i32)
    }
}

pub struct Player {
    position: Point,
    mover: Mover,
    collider: Rect,
    pivot: Point,
    grounded: bool,
    sprite: Sprite,
}

// TODO: fix filepath (use relative)
static PLAYER_PNG: &[u8] =
    include_bytes!("/Users/feresr/Workspace/learn_sdl3_gpu/game/assets/player.png");
static PLAYER_ATLAS: &str =
    include_str!("/Users/feresr/Workspace/learn_sdl3_gpu/game/assets/player.atlas");

impl Player {
    pub fn new(device: Device) -> Player {
        let texture = Texture::from_bytes(device, PLAYER_PNG);

        Self {
            position: Point::new(0, 0),
            mover: Default::default(),
            collider: Rect::new(0, 0, 8, 8),
            pivot: Point::new(-4, -8),
            grounded: false,
            sprite: Sprite::from_atlas(texture, PLAYER_ATLAS),
        }
    }

    pub fn update(&mut self, room: &Room) {
        self.sprite.update();
        self.sprite.play("IDLE");

        // Controls
        self.mover.speed.x = 0f32;
        if Keyboard::held(common::Keycode::D) {
            self.mover.speed.x = 2f32;
            self.sprite.flip_x = false;
        }
        if Keyboard::held(common::Keycode::A) {
            self.mover.speed.x = -2f32;
            self.sprite.flip_x = true;
        }

        if Keyboard::pressed(common::Keycode::W) && self.grounded {
            self.mover.speed.y = -8f32;
        }
        if Keyboard::pressed(common::Keycode::Space) && self.grounded {
            self.sprite.play("ATTACK");
        }

        // Reposition collider to self.position
        self.collider.reposition(self.position + self.pivot);
        // Calculate the integer and reminder speed to move
        let mut to_move = self.mover.get_integer_move_amount();

        let signum_y = to_move.y.signum();
        while to_move.y.abs() > 0 {
            self.collider.offset(0, signum_y);
            to_move.y -= signum_y;
            if room.collides(&self.collider) {
                self.mover.speed.y = 0f32;
                self.mover.reminder.y = 0f32;
                self.collider.offset(0, -signum_y);
                if room.collides(&self.collider) {
                    // TODO: remove this in release
                    panic!("Still colliding after y fix")
                }
                break;
            }
        }
        let signum_x = to_move.x.signum();
        while to_move.x.abs() > 0 {
            self.collider.offset(signum_x, 0);
            to_move.x -= signum_x;
            if room.collides(&self.collider) {
                self.mover.speed.x = 0f32;
                self.mover.reminder.x = 0f32;
                self.collider.offset(-signum_x, 0);
                if room.collides(&self.collider) {
                    // TODO: remove this in release
                    panic!("Still colliding after x fix")
                }
                break;
            }
        }

        // Check/Update grounded state
        let mut ground_check_collider = self.collider.clone();
        ground_check_collider.offset(0, 1);
        self.grounded = room.collides(&ground_check_collider);

        // Apply Gravity
        if !self.grounded {
            self.mover.speed.y += 0.3f32;
            self.sprite.play("JUMP");
        }

        // Apply movement
        self.position.x = self.collider.x - self.pivot.x;
        self.position.y = self.collider.y - self.pivot.y;
    }

    pub fn render(&self, batch: &mut Batch) {
        self.sprite.render(&self.position, batch);

        // DEBUG collider
        // batch.rect(
        //     [
        //         (self.position.x + self.pivot.x) as f32,
        //         (self.position.y + self.pivot.y) as f32,
        //         0f32,
        //     ],
        //     [8f32, 8f32],
        //     [255, 0, 0, 100],
        // );
        // batch.rect(
        //     [(self.position.x) as f32, (self.position.y) as f32, 0f32],
        //     [1f32, 1f32],
        //     [255, 255, 255, 100],
        // );
    }
}
