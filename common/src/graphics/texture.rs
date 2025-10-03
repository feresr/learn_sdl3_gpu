use std::{
    path::Path,
    rc::Rc,
};

use sdl3::gpu::{
    self, CopyPass, Device, Sampler, SamplerCreateInfo, TextureCreateInfo, TextureFormat,
    TextureRegion, TextureSamplerBinding, TextureTransferInfo, TransferBuffer,
};

static mut NEXT_ID: u16 = 0;

/**
 * Lightweight handle wrapping around a Texture + Sampler.
 */
#[derive(Clone)]
pub struct Texture {
    pub id: u16,
    uploaded: bool, // TODO: rename to needs_upload?
    inner: Rc<(sdl3::gpu::Texture<'static>, Sampler, TransferBuffer)>,
}

impl PartialEq for Texture {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Texture {
    pub fn from_bytes(device: Device, bytes: &[u8]) -> Self {
        let load_result = stb_image::image::load_from_memory(bytes);

        let image = match load_result {
            stb_image::image::LoadResult::Error(_) => panic!("Could not load image path"),
            stb_image::image::LoadResult::ImageU8(image) => image,
            stb_image::image::LoadResult::ImageF32(_image) => {
                panic!("Only u8 images are supported")
            }
        };

        let mut texture = Texture::new(
            device.clone(),
            image.width as u16,
            image.height as u16,
            // TextureFormat::B8g8r8a8Unorm, // TODO
            TextureFormat::R8g8b8a8Unorm,
        );

        let (_, _, transfer_buffer) = texture.inner.as_ref();
        texture.uploaded = false;
        let mut map = transfer_buffer.map(&device, true);
        let memory = map.mem_mut();
        memory[..image.data.len()].copy_from_slice(&image.data);
        map.unmap();

        return texture;
    }

    pub fn from_path<P: AsRef<Path>>(device: Device, path: P) -> Self {
        let load_result = stb_image::image::load(path);

        let image = match load_result {
            stb_image::image::LoadResult::Error(_) => panic!("Could not load image path"),
            stb_image::image::LoadResult::ImageU8(image) => image,
            stb_image::image::LoadResult::ImageF32(_image) => {
                panic!("Only u8 images are supported")
            }
        };

        let mut texture = Texture::new(
            device.clone(),
            image.width as u16,
            image.height as u16,
            // TextureFormat::B8g8r8a8Unorm, // TODO
            TextureFormat::R8g8b8a8Unorm,
        );

        let (_, _, transfer_buffer) = texture.inner.as_ref();
        texture.uploaded = false;
        let mut map = transfer_buffer.map(&device, true);
        let memory = map.mem_mut();
        memory[..image.data.len()].copy_from_slice(&image.data);
        map.unmap();

        return texture;
    }

    pub fn upload(&mut self, pass: &CopyPass) {
        if self.uploaded {
            return;
        }
        self.uploaded = true;

        let (texture, _, transfer_buffer) = self.inner.as_ref();
        pass.upload_to_gpu_texture(
            TextureTransferInfo::new()
                .with_offset(0)
                .with_pixels_per_row(texture.width() as u32)
                .with_rows_per_layer(texture.height() as u32)
                .with_transfer_buffer(&transfer_buffer),
            TextureRegion::new()
                .with_texture(&texture)
                .with_mip_level(0)
                .with_x(0)
                .with_y(0)
                .with_width(texture.width() as u32)
                .with_height(texture.height() as u32)
                .with_depth(1),
            false,
        );
    }

    pub fn new(device: Device, width: u16, height: u16, texture_format: TextureFormat) -> Self {
        let texture = device
            .create_texture(
                TextureCreateInfo::new()
                    .with_type(gpu::TextureType::_2D)
                    .with_format(texture_format)
                    .with_usage(gpu::TextureUsage::SAMPLER | gpu::TextureUsage::COLOR_TARGET)
                    .with_width(width as u32)
                    .with_height(height as u32)
                    .with_layer_count_or_depth(1)
                    .with_num_levels(1)
                    .with_sample_count(gpu::SampleCount::NoMultiSampling),
            )
            .expect("Could not create texture.");

        let sampler = device
            .create_sampler(
                SamplerCreateInfo::new()
                    .with_min_filter(gpu::Filter::Nearest)
                    .with_mag_filter(gpu::Filter::Nearest)
                    .with_mipmap_mode(gpu::SamplerMipmapMode::Nearest)
                    .with_address_mode_u(gpu::SamplerAddressMode::ClampToEdge)
                    .with_address_mode_v(gpu::SamplerAddressMode::ClampToEdge)
                    .with_address_mode_w(gpu::SamplerAddressMode::ClampToEdge)
                    .with_enable_compare(false),
            )
            .expect("Could not create sampler");

        let channels: u32 = 4;
        let size = (width as u32 * height as u32) * channels;
        let transfer_buffer = device
            .create_transfer_buffer()
            .with_usage(sdl3::sys::gpu::SDL_GPUTransferBufferUsage::UPLOAD)
            .with_size(size as u32)
            .build()
            .expect("Could not build transfer buffer");

        let id = unsafe {
            let id = NEXT_ID;
            NEXT_ID += 1;
            id
        };

        return Texture {
            id,
            inner: Rc::new((texture, sampler, transfer_buffer)),
            uploaded: true, // Offscreen render_targets don't need to be 'uploaded'
        };
    }

    pub fn inner(&self) -> &sdl3::gpu::Texture<'static> {
        &self.inner.0
    }

    pub fn width(&self) -> u32 {
        let (texture, _, _) = self.inner.as_ref();
        texture.width()
    }
    
    pub fn height(&self) -> u32 {
        let (texture, _, _) = self.inner.as_ref();
        texture.height()
    }

    pub fn bindings(&self) -> TextureSamplerBinding {
        let (texture, sampler, _) = self.inner.as_ref();
        return TextureSamplerBinding::new()
            .with_sampler(sampler)
            .with_texture(texture);
    }
}
