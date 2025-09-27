use sdl3::gpu::Device;

use crate::{
    graphics::{batch::Batch, texture::Texture},
    ui::window::Window,
    utils::{font_atlas::FontAtlas, glyph::GlyphData},
};

const MAX_WINDOWS: usize = 8;
static FONT_ATLAS: &[u8; 6516] =
    include_bytes!("/Users/feresr/Workspace/learn_sdl3_gpu/common/src/ui/Iosevka/Iosevka.png");
static FONT_GLYPH_DATA_FNT: &'static str =
    include_str!("/Users/feresr/Workspace/learn_sdl3_gpu/common/src/ui/Iosevka/Iosevka.txt");

pub struct Gui {
    windows: [Window; MAX_WINDOWS],
    window_count: usize,
    atlas: FontAtlas,
    window_cursor: f32,
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
            window_cursor: 0f32,
        }
    }

    fn get() -> &'static mut Gui {
        unsafe { &mut *GUI }
    }

    // TODO: Make windows minimize when tapping header (show no body)
    pub fn window(title: &'static str) -> &'static mut Window {
        let instance = Self::get();

        let index = instance.window_count;
        instance.window_count += 1;

        let window = &mut instance.windows[index];
        window.title = title;
        if window.position.x == 0f32 {
            window.position.x = instance.window_cursor;
            instance.window_cursor += 122f32; // TODO constant
        }

        window
    }

    /**
     * This method acts on the previous frame state.
     * (1-frame behind to detect input before anything else in the game)
     *
     * 1. Gui::update() is called at the begginig of the frame to detect user inputs and capture clicks, then clears all windows.
     * 2. Gui.window() adds new windows + widgets as needed (window.add_widget(..)) checks for click (none on the first frame)
     * 3. Gui::draw() renders windows at the end of the frame.
     *
     * Example:
     *
     * -- FRAME 1 ---
     * 1. Update: On the very first pass, there will be no windows to update. No windows to clear           [ ]
     * 2. The game adds window A with widget B -> checks for click -> no click detected                     [A]
     * 3. Windows are drawn.                                                                                [A]
     * -- frame 2 --- (user clicks on widget)
     * 4. Update: We detect window clicks and store click x,y. Clear Window A                               [A(click x,y)]
     * 5. The game adds window A with widget B -> checks for click -> click detected!                       [A(click x,y)]
     * 6. Windows are drawn again (clicks are cleared )                                                     [A]
     * -- FRAME 3 ---
     * 7. Update, Detext clicks, clear windows                                                              [ ]
     * 8. The game adds window A with widget B -> checks for click -> no click detectd                      [A]
     *
     */
    pub fn update() {
        let instance = Self::get();
        let window_count = instance.window_count;

        // Iterate backwards (from foreground to background) to detect hover/drag inputs.
        // The most recently added window gets priority for input events (drawn on top).
        for i in (0..window_count).rev() {
            let window = &mut instance.windows[i];
            if window.update() {
                // TODO: Focused windows don't work properly (other window rendering order changes)
            }
        }

        // Iterate forwards (from background to foreground) to draw the windows.
        // The most recently added window is drawn last, appearing on top of others.
        for i in 0..window_count {
            let window = &mut instance.windows[i];
            window.clear();
        }
        instance.window_count = 0;
    }

    pub fn draw(batch: &mut Batch) {
        let instance = Self::get();
        let window_count = instance.window_count;

        // Iterate forwards (from background to foreground) to draw the windows.
        // The most recently added window is drawn last, appearing on top of others.
        for i in 0..window_count {
            let window = &mut instance.windows[i];
            window.draw(batch, &instance.atlas);
        }
    }
}

pub static mut GUI: *mut Gui = std::ptr::null_mut();
