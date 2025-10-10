use common::{Point, Rect, graphics::render_target::RenderTarget};

use crate::{player::Player, world::World};

pub struct Camera {
    viewport: Rect,
}

impl Default for Camera {
    fn default() -> Self {
        let rect = Rect::new(0, 0, 320, 180);
        Self { viewport: rect }
    }
}

impl Camera {
    pub fn update(&mut self, target: &mut RenderTarget, player: &Player, world: &World) {
        // Find the current room the player is in.
        let position = player.get_position();
        let current_room = world.rooms.get_cell_at_position(
            position.x as usize,
            (position.y + 4) as usize, // TODO 4 is the offset between Romo size and scren size
        );
        // Check if we need to move the camera
        if self.viewport.x == current_room.position_in_world.x
            && self.viewport.y == current_room.position_in_world.y
        {
            return;
        }
        // If so, reposition the camera and update projection
        let top_left = Point::new(
            current_room.position_in_world.x as i32,
            current_room.position_in_world.y as i32,
        );
        self.viewport.reposition(top_left);
        *target.projection_mut() = glm::ortho(
            self.viewport.left() as f32,
            self.viewport.right() as f32,
            self.viewport.bottom() as f32,
            self.viewport.top() as f32,
            -1f32,
            1f32,
        );
    }
}
