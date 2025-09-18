use common::game_memory::GameMemory;
use sdl3::sys::{
    loadso::{SDL_LoadFunction, SDL_LoadObject, SDL_SharedObject, SDL_UnloadObject},
    stdinc::SDL_FunctionPointer,
};

use std::ffi::CStr;
use std::ffi::CString;

/* Loads the game dynamically, finds and exposes a reference to its update function */
pub struct GameDll {
    handle: *mut SDL_SharedObject,
    update: extern "C" fn(*mut GameMemory),
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

impl GameDll {
    pub fn load() -> Self {
        let update: extern "C" fn(*mut GameMemory);
        let handle;

        unsafe {
            handle = SDL_LoadObject(LIB_PATH.as_ptr());
            if handle.is_null() {
                panic!("Failed to load DLL");
            }

            let fn_name = CString::new("update_game").unwrap();
            let func: SDL_FunctionPointer = SDL_LoadFunction(handle, fn_name.as_ptr());
            if func.is_none() {
                panic!("Failed to find symbol");
            }
            update = std::mem::transmute(func);
        }

        return GameDll { handle, update };
    }

    pub fn update(&self, game_memory: &mut GameMemory) {
        let ptr: *mut GameMemory = game_memory;
        (self.update)(ptr);
    }
}

impl Drop for GameDll {
    fn drop(&mut self) {
        unsafe {
            SDL_UnloadObject(self.handle);
        }
    }
}
