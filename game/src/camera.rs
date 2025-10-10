use common::{graphics::IDENTITY, utils::create_transform_inplace, Point};

use crate::room::Room;

pub struct Camera {
    projection: glm::Mat4,
    current_position : Point,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            projection: IDENTITY,
            current_position: Point::new(0, 0),
        }
    }
}

impl Camera {

    pub fn projection(&self) -> &glm::Mat4 {
        &self.projection
    }

    pub fn update(&mut self, current_room: &Room) {
        if self.current_position == current_room.position_in_world {
            return;
        }
        self.current_position = current_room.position_in_world;
        self.projection.fill_with_identity();
        create_transform_inplace(
            &mut self.projection,
            glm::vec2(
                -current_room.position_in_world.x as f32,
                -current_room.position_in_world.y as f32,
            ),
            glm::vec2(0f32, 0f32),
            glm::vec2(1f32, 1f32),
        );
    }
}
