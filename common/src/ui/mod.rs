use sdl3::gpu::Device;

use crate::{
    graphics::{batch::Batch, texture::Texture},
    ui::window::Window,
    utils::{glyph::GlyphData, font_atlas::FontAtlas},
};

pub mod widget;
pub mod window;

// TODO: move this to GUI.rs
const WINDOW_TO_WINDOW_PADDING: f32 = 8f32;
const MAX_WINDOWS: usize = 8;
static FONT_ATLAS: &[u8; 6516] =
    include_bytes!("/Users/feresr/Workspace/learn_sdl3_gpu/common/src/ui/Iosevka/Iosevka.png");
static FONT_GLYPH_DATA_FNT: &'static str =
    include_str!("/Users/feresr/Workspace/learn_sdl3_gpu/common/src/ui/Iosevka/Iosevka.txt");

pub struct Gui {
    windows: [Window; MAX_WINDOWS],
    window_count: usize,
    atlas: FontAtlas,
    arrange_windows: bool, // Arrange windows side by side on first draw
}

impl Gui {
    pub fn new(device: Device) -> Self {
        let texture_atlas = Texture::from_bytes(device, FONT_ATLAS);
        let glyph_data = GlyphData::from_fnt_file(FONT_GLYPH_DATA_FNT);
        let atlas = FontAtlas::new(texture_atlas, glyph_data);
        Self {
            windows: Default::default(),
            window_count: Default::default(),
            atlas,
            arrange_windows: true,
        }
    }

    fn get() -> &'static mut Gui {
        unsafe { &mut *GUI }
    }

    pub fn window(title: &'static str) -> &'static mut Window {
        let instance = Self::get();

        let index = instance.window_count;
        instance.window_count += 1;

        let window = &mut instance.windows[index];
        window.title = title;

        window
    }

    pub fn draw(batch: &mut Batch) {
        let instance = Self::get();
        let window_count = instance.window_count;

        // Only one window can be dragged at a time.
        let mut drag_allowed = true;

        let mut window_layout_cursor_x = 0f32;
        // Iterate backwards (from foreground to background) to detect hover/drag inputs.
        // The most recently added window gets priority for input events (drawn on top).
        for i in (0..window_count).rev() {
            let window = &mut instance.windows[i];
            let capture_drag = window.update(drag_allowed, &instance.atlas);
            // If this window captured the drag, prevent underlying windows from dragging.
            // Once a window captures drag, drag_allowed remains false for remaining windows.
            drag_allowed = !capture_drag && drag_allowed;

            if instance.arrange_windows {
                window.position.x = window_layout_cursor_x;
                window_layout_cursor_x += window.size.x + WINDOW_TO_WINDOW_PADDING;
            }
        }

        // Only arrange windows on first draw
        instance.arrange_windows = false;

        // Iterate forwards (from background to foreground) to draw the windows.
        // The most recently added window is drawn last, appearing on top of others.
        for i in 0..window_count {
            let window = &mut instance.windows[i];
            window.draw(batch, &instance.atlas);
            window.clear();
        }

        instance.window_count = 0;
    }
}

pub static mut GUI: *mut Gui = std::ptr::null_mut();
