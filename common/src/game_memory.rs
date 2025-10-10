use std::ffi::c_void;

use sdl3::sys::stdinc::{SDL_free, SDL_malloc};

#[repr(C)]
pub struct GameMemory {
    pub initialized: bool,
    pub storage: *mut c_void,
    pub quit: bool,
}

impl GameMemory {

    #[cfg(not(debug_assertions))]
    pub const GAME_MEMORY: usize = 1024 * 2;

    // Debug ImGui requires more memory 
    #[cfg(debug_assertions)]
    pub const GAME_MEMORY: usize = 1024 * 64;

    pub fn default() -> Self {
        let storage = unsafe { SDL_malloc(GameMemory::GAME_MEMORY) };

        Self {
            initialized: false,
            storage,
            quit: false,
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
