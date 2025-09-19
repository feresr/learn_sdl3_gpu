use common::{
    Device,
    game_memory::GameMemory,
    graphics::{batch::Batch, render_target::RenderTarget},
    input::{
        keyboard::Keyboard,
        mouse::Mouse,
    },
};
use sdl3::sys::{
    loadso::{SDL_LoadFunction, SDL_LoadObject, SDL_SharedObject, SDL_UnloadObject},
    stdinc::SDL_FunctionPointer,
};

use std::ffi::CStr;

type UpdateFn =
    extern "C" fn(&mut GameMemory, &mut Batch, &RenderTarget, &Keyboard, &Mouse, &Device);

/* Loads the game dynamically, finds and exposes a reference to its update function */
pub struct GameDll {
    handle: *mut SDL_SharedObject,
    update: UpdateFn,
}

#[cfg(all(debug_assertions, target_os = "macos"))]
const LIB_PATH: &CStr = c"target/debug/libgame.dylib";

#[cfg(all(not(debug_assertions), target_os = "macos"))]
const LIB_PATH: &CStr = c"target/release/libgame.dylib";

#[cfg(all(debug_assertions, target_os = "windows"))]
const LIB_PATH: &CStr = c"target/debug/game.dll";

#[cfg(all(not(debug_assertions), target_os = "windows"))]
const LIB_PATH: &CStr = c"target/release/game.dll";

#[cfg(all(debug_assertions, target_os = "linux"))]
const LIB_PATH: &CStr = c"target/debug/libgame.so";

#[cfg(all(not(debug_assertions), target_os = "linux"))]
const LIB_PATH: &CStr = c"target/release/libgame.so";

const UPDATE_GAME: &CStr = c"update_game";

impl GameDll {
    pub fn load() -> Self {
        let update: UpdateFn;
        let handle;

        unsafe {
            handle = SDL_LoadObject(LIB_PATH.as_ptr());
            if handle.is_null() {
                panic!("Failed to load DLL");
            }

            let func: SDL_FunctionPointer = SDL_LoadFunction(handle, UPDATE_GAME.as_ptr());
            if func.is_none() {
                panic!("Failed to find symbol");
            }
            update = std::mem::transmute(func);
        }

        return GameDll { handle, update };
    }

    pub fn update(
        &self,
        game_memory: &mut GameMemory,
        batch: &mut Batch,
        screen_target: &RenderTarget,
        keyboard: &Keyboard,
        mouse: &Mouse,
        device: &Device,
    ) {
        (self.update)(game_memory, batch, screen_target, keyboard, mouse, device);
    }
}

impl Drop for GameDll {
    fn drop(&mut self) {
        unsafe {
            SDL_UnloadObject(self.handle);
        }
    }
}
