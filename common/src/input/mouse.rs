use sdl3::mouse::MouseButton;

#[derive(Default, Debug)]
pub struct Mouse {
    position: glm::Vec2,
    wheel: glm::Vec2,
    position_rel: glm::Vec2,
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
    pub fn left_clicked() -> bool {
        Self::get().left
    }
    pub fn left_held() -> bool {
        Self::get().left_held
    }
    pub fn right_clicked() -> bool {
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

    pub fn position_rel() -> glm::Vec2 {
        Self::get().position_rel
    }

    pub fn wheel() -> glm::Vec2 {
        Self::get().wheel
    }

    pub fn mouse_button_down(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => {
                self.left = true;
                self.left_held = true;
            }
            MouseButton::Right => {
                self.right = true;
                self.right_held = true;
            }
            _ => {}
        }
    }

    pub fn set_wheel(&mut self, x: f32, y: f32) {
        self.wheel.x = x;
        self.wheel.y = y;
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
        self.position.x = x;
        self.position.y = y;
        self.position_rel.x += xrel;
        self.position_rel.y += yrel;
    }

    /**
     * The relative position is only valid for one frame
     * If the mouse does not move from frame 0 to frame 1 no MotionEvent will trigger...
     * Which means no Mouse.set_position(x, y, xrel, yrel) invokation.
     * Therefore, we need to clear this manually at the end of the frame.
     */
    pub fn clear_relative_position(&mut self) {
        self.position_rel.scale_mut(0f32);
    }
    pub fn clear_button_pressed(&mut self) {
        self.left = false;
        self.right = false;
    }
}

pub static mut MOUSE: *const Mouse = std::ptr::null_mut();
