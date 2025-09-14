use std::ffi::c_void;

use sdl3::sys::stdinc::{SDL_free, SDL_malloc};

const GAME_MEMORY: usize = 1024 * 16;

#[repr(C)]
pub struct GameMemory {
    pub initialized: bool,
    pub storage_size: usize,
    pub storage: *mut c_void,
}

impl GameMemory {
    pub fn default() -> Self {
        let storage = unsafe { SDL_malloc(GAME_MEMORY) };

        Self {
            initialized: false,
            storage_size: GAME_MEMORY,
            storage,
        }
    }
}

impl Drop for GameMemory {
    fn drop(&mut self) {
        if self.initialized {
            self.initialized = false;
            unsafe { SDL_free(self.storage) }
        }
    }
}
