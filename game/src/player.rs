use common::{Point, Rect, graphics::batch::Batch, input::keyboard::Keyboard};

use crate::room::Room;

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
}

impl Player {
    pub fn new() -> Player {
        Self {
            position: Point::new(0, 0),
            mover: Default::default(),
            collider: Rect::new(0, 0, 8, 8),
            pivot: Point::new(-4, -4),
            grounded: false,
        }
    }

    pub fn update(&mut self, room: &Room) {
        // Controls
        self.mover.speed.x = 0f32;
        if Keyboard::held(common::Keycode::D) {
            self.mover.speed.x = 2f32;
        }
        if Keyboard::held(common::Keycode::A) {
            self.mover.speed.x = -2f32;
        }

        if Keyboard::pressed(common::Keycode::W) && self.grounded {
            self.mover.speed.y = -8f32;
        }

        // Reposition collider to self.position
        self.collider.reposition(self.position + self.pivot);

        let mut ground_check_collider = self.collider.clone();
        ground_check_collider.offset(0, 1);
        self.grounded = room.collides(&ground_check_collider);

        // Apply Gravity
        if !self.grounded {
            self.mover.speed.y += 0.3f32;
        }

        // Calculate the integer and reminder speed to move
        let mut to_move = self.mover.get_integer_move_amount();

        while to_move.y.abs() > 0 {
            if room.collides(&self.collider) {
                self.mover.speed.y = 0f32;
                self.collider.y -= to_move.y.signum(); // Backs out of collision
                break;
            }
            self.collider.offset(0, to_move.y.signum());
            to_move.y -= to_move.y.signum();
        }
        while to_move.x.abs() > 0 {
            if room.collides(&self.collider) {
                self.mover.speed.x = 0f32;
                self.collider.x -= to_move.x.signum();
                break;
            }
            self.collider.offset(to_move.x.signum(), 0);
            to_move.x -= to_move.x.signum();
        }

        // Apply movement
        self.position.x = self.collider.x - self.pivot.x;
        self.position.y = self.collider.y - self.pivot.y;
    }

    pub fn render(&self, batch: &mut Batch) {
        batch.circle(
            [self.position.x as f32, self.position.y as f32],
            12f32,
            22,
            [0, 185, 20, 255],
        );
    }
}
