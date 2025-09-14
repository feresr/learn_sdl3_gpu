use common::game_memory::GameMemory;
use sdl3::sys::{
    loadso::{SDL_LoadFunction, SDL_LoadObject, SDL_SharedObject, SDL_UnloadObject},
    stdinc::SDL_FunctionPointer,
};
use std::ffi::CString;

/* Loads the game dynamically, finds and exposes a reference to its update function */
pub struct GameDll {
    handle: *mut SDL_SharedObject,
    update: extern "C" fn(*mut GameMemory),
}

impl GameDll {
    pub fn load() -> Self {
        let update: extern "C" fn(*mut GameMemory);
        let handle;

        unsafe {
            // TODO: support dylib dll so
            let lib_path = CString::new("target/debug/libgame.dylib").unwrap();
            handle = SDL_LoadObject(lib_path.as_ptr());
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
