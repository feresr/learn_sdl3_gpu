use crate::{graphics::batch::Batch, ui::window::Window};

pub mod window;

const MAX_WINDOWS: usize = 8;

#[derive(Default)]
pub struct Gui {
    windows: [Window; MAX_WINDOWS],
    window_count: usize,
}

impl Gui {
    fn get() -> &'static mut Gui {
        unsafe { &mut *GUI }
    }

    pub fn window(width: f32, height: f32) -> &'static mut Window {
        let instance = Self::get();
        let index = instance.window_count;
        instance.window_count += 1;
        let window = &mut instance.windows[index];
        window.size.x = width;
        window.size.y = height;
        window
    }

    pub fn draw(batch: &mut Batch) {
        let instance = Self::get();
        let window_count = instance.window_count;
        for i in 0..window_count {
            let window = &mut instance.windows[i];
            window.update();
            window.draw(batch);
            window.clear();
        }
        instance.window_count = 0;
    }
}

pub static mut GUI: *mut Gui = std::ptr::null_mut();
