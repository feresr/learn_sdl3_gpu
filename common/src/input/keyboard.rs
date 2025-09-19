use sdl3::keyboard::Keycode;

#[derive(Default)]
pub struct Keyboard {
    pressed_keys: std::collections::HashSet<Keycode>,
    held_keys: std::collections::HashSet<Keycode>,
}

impl Keyboard {

    fn get() -> &'static Keyboard {
        unsafe { &*KEYBOARD }
    }

    // ---- Static Methods meant to be invoked by the game dll

    pub fn pressed(key: Keycode) -> bool {
        Self::get().pressed_keys.contains(&key)
    }

    pub fn held(key: Keycode) -> bool {
        Self::get().held_keys.contains(&key)
    }

    // ---- Methods below are meant to be invoked by the runtime

    /**  
     * This will be invoked multiple times if the user holds the key down
     * On the first iteration we set pressed = !held and held = true
     * Resulting in pressed = true, held = true on the first pass
     * and pressed = false, held = true in subsequent passes.
     */
    pub fn press(&mut self, key: Keycode) {
        if self.pressed_keys.contains(&key) {
            self.pressed_keys.remove(&key);
        } else {
            self.pressed_keys.insert(key);
        }
        self.held_keys.insert(key);
    }

    pub fn release(&mut self, key: &Keycode) {
        self.pressed_keys.remove(key);
        self.held_keys.remove(key);
    }

    pub fn clear_pressed(&mut self) {
        self.pressed_keys.clear();
    }
}

pub static mut KEYBOARD: *const Keyboard = std::ptr::null_mut();
