use common::game_memory::GameMemory;

#[unsafe(no_mangle)]
pub extern "C" fn update_game(game_memory: *mut GameMemory) {
    let memory: &mut GameMemory = unsafe { &mut *game_memory };
    if !memory.initialized {
        memory.initialized = true;
        println!("Initialising")
    }
}
