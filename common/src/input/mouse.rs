use sdl3::mouse::MouseButton;

#[derive(Default, Debug)]
pub struct Mouse {
    position: glm::Vec2,
    wheel: (i32, i32),
    position_rel: (i32, i32),
    left: bool,
    right: bool,
    left_held: bool,
    right_held: bool,
}

impl Mouse {
    fn get() -> &'static Mouse {
        unsafe { &*MOUSE }
    }

    // Getters
    pub fn left_pressed() -> bool {
        Self::get().left
    }
    pub fn left_held() -> bool {
        Self::get().left_held
    }
    pub fn right_pressed() -> bool {
        Self::get().right
    }
    pub fn right_held() -> bool {
        Self::get().right_held
    }

    pub fn position() -> glm::Vec2 {
        Self::get().position
    }

    pub fn position_projected(projection: &glm::Mat4) -> glm::Vec2 {
        let position = Self::get().position;
        (projection * glm::vec4(position.x, position.y, 0f32, 1.0f32)).xy()
    }

    pub fn position_rel() -> (i32, i32) {
        Self::get().position_rel
    }

    pub fn wheel() -> (i32, i32) {
        Self::get().wheel
    }

    pub fn mouse_button_down(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => {
                self.left = !self.left_held;
                self.left_held = true;
            }
            MouseButton::Right => {
                self.right = !self.right_held;
                self.right = true;
            }
            _ => {}
        }
    }

    pub fn set_wheel(&mut self, x: i32, y: i32) {
        self.wheel = (x, y);
    }

    pub fn mouse_button_up(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => {
                self.left = false;
                self.left_held = false;
            }
            MouseButton::Right => {
                self.right = false;
                self.right_held = false;
            }
            _ => {}
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32, xrel: f32, yrel: f32) {
        self.position = glm::vec2(x, y);
        self.position_rel = (xrel as i32, yrel as i32);
    }
}

pub static mut MOUSE: *const Mouse = std::ptr::null_mut();
