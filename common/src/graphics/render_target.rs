use std::mem;

use sdl3::{
    gpu::{ColorTargetInfo, LoadOp, StoreOp},
    pixels::Color,
};

use crate::graphics::texture::Texture;

enum BackingTexture {
    Screen(Option<sdl3::gpu::Texture<'static>>),
    Texture(Texture),
}

const CLEAR_COLOR_SCREEN: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};
const CLEAR_COLOR: Color = Color {
    r: 25,
    g: 25,
    b: 25,
    a: 255,
};

pub struct RenderTarget {
    pub width: u32,
    pub height: u32,
    projection: Option<glm::Mat4>,
    texture: BackingTexture,
}

impl RenderTarget {
    pub fn empty() -> Self {
        RenderTarget {
            projection: None,
            width: 0,
            height: 0,
            texture: BackingTexture::Screen(None),
        }
    }

    pub fn color(&self) -> Texture {
        if let BackingTexture::Texture(texture) = &self.texture {
            return texture.clone();
        }
        panic!("Trying to get color texture from BackingTexture::Screen")
    }

    /**
     * This API is only for the screen texture and should not be used
     * for regular render targets.
     */
    pub fn set_texture(&mut self, texture: sdl3::gpu::Texture<'_>) {
        // Only sets the projection matrix once for performance
        if self.projection.is_none() {
            self.resize(texture.width() as i32, texture.height() as i32)
        }

        // Unsafe:
        // RenderTarget lives longer (outside main_loop) than the swapchain_texture (inside main_loop)
        //
        // To avoid recreating the projection matrix on every frame, we use unsafe to treat
        // Texture<'loop> as Texture<'static>, but we guarantee safety by clearing the texture reference
        // before it becomes invalid.
        //
        //   let render_target = RenderTarget::with_matrix(matrix); // 'static lifetime (long)
        //   loop {
        //       let cmd = ...
        //       render_target.set_texture(cmd.swapchain_texture()); // 'loop lifetime (short)
        //       batch.render(render_target);
        //       render_target.clear_texture(); // Clear before texture is dropped
        //   }
        unsafe {
            self.texture = BackingTexture::Screen(Some(mem::transmute(texture)));
        }
    }

    /**
     * This API is only for the screen texture and should not be used
     * for regular render targets.
     */
    pub fn clear_texture(&mut self) {
        if let BackingTexture::Texture(_) = self.texture {
            panic!("clear_texture is a Screen only API")
        }
        self.texture = BackingTexture::Screen(None);
    }

    pub fn new(texture: Texture) -> Self {
        let projection = glm::ortho(
            0.0f32,
            texture.width() as f32,
            texture.height() as f32,
            0.0f32,
            -1.0f32,
            1.0f32,
        );

        RenderTarget {
            projection: Some(projection),
            width: texture.width(),
            height: texture.height(),
            texture: BackingTexture::Texture(texture),
        }
    }

    pub fn projection(&self) -> &glm::Mat4 {
        return self
            .projection
            .as_ref()
            .expect("Missing projection: Empty RenderTarget");
    }

    pub fn color_target_info(&self) -> ColorTargetInfo {
        let texture = match &self.texture {
            BackingTexture::Screen(texture) => texture
                .as_ref()
                .expect("Missing texture: Call to color_target_info on an empty RenderTarget"),
            BackingTexture::Texture(texture) => texture.inner(),
        };

        let clear_color = match &self.texture {
            BackingTexture::Screen(_) => CLEAR_COLOR_SCREEN,
            BackingTexture::Texture(_) => CLEAR_COLOR,
        };

        ColorTargetInfo::default()
            .with_texture(texture)
            .with_store_op(StoreOp::STORE)
            .with_load_op(LoadOp::CLEAR)
            .with_clear_color(clear_color)
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        if let BackingTexture::Texture(_) = self.texture {
            panic!("Trying to resize non BackingTexture::Screen Render target")
        }
        let projection = glm::ortho(0.0f32, width as f32, height as f32, 0.0f32, -1.0f32, 1.0f32);
        self.width = width as u32;
        self.height = height as u32;
        self.projection = Some(projection);
    }
}

impl Drop for RenderTarget {
    fn drop(&mut self) {
        if let BackingTexture::Screen(_) = self.texture {
            self.clear_texture();
        }
    }
}
