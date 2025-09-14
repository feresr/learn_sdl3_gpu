use std::{path::Path, rc::Rc};

use sdl3::gpu::{
    self, CopyPass, Device, Sampler, SamplerCreateInfo, TextureCreateInfo, TextureFormat,
    TextureRegion, TextureSamplerBinding, TextureTransferInfo, TransferBuffer,
};

pub struct Texture {
    pub width: i32,
    pub height: i32,
    pub format: TextureFormat,
    inner_texture: sdl3::gpu::Texture<'static>,
    inner_sampler: Sampler,
    transfer_buffer: TransferBuffer,
    uploaded: bool,
}

impl Texture {
    pub fn from_path<P: AsRef<Path>>(device: Device, path: P) -> Self {
        let load_result = stb_image::image::load(path);

        let image = match load_result {
            stb_image::image::LoadResult::Error(_) => panic!("Could not load image path"),
            stb_image::image::LoadResult::ImageU8(image) => image,
            stb_image::image::LoadResult::ImageF32(image) => panic!("Only u8 images are supported"),
        };

        let texture = Texture::new(
            device.clone(),
            image.width as i32,
            image.height as i32,
            // TextureFormat::B8g8r8a8Unorm, // TODO
            TextureFormat::R8g8b8a8Unorm,
        );

        let mut map = texture.transfer_buffer.map(&device, true);
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

        pass.upload_to_gpu_texture(
            TextureTransferInfo::new()
                .with_offset(0)
                .with_pixels_per_row(self.width as u32)
                .with_rows_per_layer(self.height as u32)
                .with_transfer_buffer(&self.transfer_buffer),
            TextureRegion::new()
                .with_texture(&self.inner_texture)
                .with_mip_level(0)
                .with_x(0)
                .with_y(0)
                .with_width(self.width as u32)
                .with_height(self.height as u32)
                .with_depth(1),
            false,
        );
    }

    fn new(device: Device, width: i32, height: i32, texture_format: TextureFormat) -> Self {
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

        let channels = 4;
        let size = width * height * channels;
        let transfer_buffer = device
            .create_transfer_buffer()
            .with_usage(sdl3::sys::gpu::SDL_GPUTransferBufferUsage::UPLOAD)
            .with_size(size as u32)
            .build()
            .expect("Could not build transfer buffer");

        return Texture {
            width,
            height,
            format: texture_format,
            inner_texture: texture,
            inner_sampler: sampler,
            transfer_buffer: transfer_buffer,
            uploaded: false,
        };
    }

    pub fn bindings(&self) -> TextureSamplerBinding {
        return TextureSamplerBinding::new()
            .with_sampler(&self.inner_sampler)
            .with_texture(&self.inner_texture);
    }
}
