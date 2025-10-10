use std::cell::Cell;

use sdl3::mouse::MouseButton;

#[derive(Default, Debug)]
pub struct Mouse {
    position: glm::Vec2,
    wheel: glm::Vec2,
    position_delta: glm::Vec2,
    // Cell: Mouse is immutable from .dll perspective (game should not update input) but,
    // consume_click() needs to set this to false to avoid further event propagation.
    left: Cell<bool>,
    right: Cell<bool>,
    right_held: Cell<bool>,
    left_history: Cell<u8>, // Interpret this as binary 00000000 means the left was not pressed in the past 8 frames
    left_consumed: Cell<bool>, // Don't left_clicked left_held will return true for the remainder of this frame
}

impl Mouse {
    fn get() -> &'static Mouse {
        unsafe { &*MOUSE }
    }

    // Getters
    /**
     * Find a pattern of the form 0110 (mouse held down and released quickly)
     */
    pub fn left_clicked() -> bool {
        let instace = Self::get();
        if instace.left_consumed.get() {
            return false;
        }
        let history = instace.left_history.get();
        history & 0b111 == 0b010
            || history & 0b1111 == 0b0110
            || history & 0b11111 == 0b01110
            || history & 0b111111 == 0b011110
            || history & 0b1111111 == 0b0111110
            || history & 0b11111111 == 0b01111110
    }

    /**
     * Find a pattern of the form 1111 (moust held down for at least 4 frames)
     */
    pub fn left_held() -> bool {
        let instace = Self::get();
        if instace.left_consumed.get() {
            return false;
        }
        instace.left_history.get() & 0b1111 == 0b1111
    }

    // TODO: refactor right_clicked to work like left_history
    pub fn right_clicked() -> bool {
        Self::get().right.get()
    }
    pub fn right_held() -> bool {
        Self::get().right_held.get()
    }

    /**
     * Prevents this event from propagating further.
     */
    pub fn consume_left() {
        Mouse::get().left_consumed.set(true);
    }

    pub fn consume_right() {
        Mouse::get().right.set(false);
    }

    pub fn position() -> glm::Vec2 {
        Self::get().position
    }

    pub fn position_relative(to: glm::Vec2) -> glm::Vec2 {
        Self::get().position - to
    }

    pub fn position_projected(projection: &glm::Mat4) -> glm::Vec2 {
        let position = Self::get().position;
        (projection * glm::vec4(position.x, position.y, 0f32, 1.0f32)).xy()
    }

    /**
     * Returns the position delta from the previous frame
     */
    pub fn position_delta() -> glm::Vec2 {
        Self::get().position_delta
    }

    pub fn wheel() -> glm::Vec2 {
        Self::get().wheel
    }

    pub fn mouse_button_down(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => {
                self.left.set(true);
            }
            MouseButton::Right => {
                self.right.set(true);
                self.right_held.set(true);
            }
            _ => {}
        }
    }

    pub fn mouse_button_up(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => {
                self.left.set(false);
            }
            MouseButton::Right => {
                self.right.set(false);
                self.right_held.set(false);
            }
            _ => {}
        }
    }

    pub fn set_wheel(&mut self, x: f32, y: f32) {
        self.wheel.x = x;
        self.wheel.y = y;
    }

    pub fn set_position(&mut self, x: f32, y: f32, xdelta: f32, ydelta: f32) {
        self.position.x = x;
        self.position.y = y;
        self.position_delta.x += xdelta;
        self.position_delta.y += ydelta;
    }

    /**
     * The relative position is only valid for one frame
     * If the mouse does not move from frame 0 to frame 1 no MotionEvent will trigger.
     * Which means no Mouse.set_position(x, y, xrel, yrel) invokation.
     * Therefore, we need to clear this manually at the end of the frame.
     */
    pub fn clear_position_delta(&mut self) {
        self.position_delta.scale_mut(0f32);
    }

    pub fn clear_button_pressed(&mut self) {
        self.left_consumed.set(false);
        self.right.set(false);
        // Shift left and write 0 or 1 depending on the statuse of the left button
        self.left_history
            .set(self.left_history.get() << 1 | self.left.get() as u8);
    }

    pub fn clear_wheel(&mut self) {
        self.wheel.x = 0f32;
        self.wheel.y = 0f32;
    }
}

pub static mut MOUSE: *const Mouse = std::ptr::null_mut();
