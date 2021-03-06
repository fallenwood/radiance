use image::RgbaImage;
use std::path::PathBuf;

pub struct Texture {
    image: RgbaImage,
}

impl Texture {
    pub fn new(path: &PathBuf) -> Self {
        let image = match image::open(path) {
            Ok(img) => img,
            Err(_) => {
                image::load_from_memory(radiance_assets::TEXTURE_MISSING_TEXTURE_FILE).unwrap()
            }
        }
        .to_rgba();

        Self { image }
    }

    pub fn new_with_iamge(image: RgbaImage) -> Self {
        Self { image }
    }

    pub fn data(&self) -> &[u8] {
        self.image.as_ref()
    }

    pub fn width(&self) -> u32 {
        self.image.width()
    }

    pub fn height(&self) -> u32 {
        self.image.height()
    }
}
