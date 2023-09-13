use std::error::Error;
use std::path::Path;

use image::imageops::{resize, FilterType};
use image::io::Reader;
use image::{GenericImage, GenericImageView, RgbImage};

use crate::char::VGAChar;
use crate::chunk::Chunk;
use crate::constants::*;

pub struct Image {
    pub image: RgbImage,
}

impl Image {
    pub fn new(path: &Path) -> Result<Self, Box<dyn Error>> {
        let image = Reader::open(path)?.decode()?.into_rgb8();

        // triangle (#2) is ~ 0.8 times slower than Nearest (#1) but but way better than Nearest and faster than the other ones.
        Ok(Image {
            image: resize(
                &image,
                VGA_PIXEL_WIDTH,
                VGA_PIXEL_HEIGHT,
                FilterType::Triangle,
            ),
        })
    }

    // no threading here because doing it at the thread/frame stage is much more efficient
    pub fn process_image(&self) -> ProcessedImage {
        let mut chars = [[VGAChar::uninit(); VGA_CHAR_HEIGHT]; VGA_CHAR_WIDTH];

        for y in 0..VGA_CHAR_HEIGHT {
            for x in 0..VGA_CHAR_WIDTH {
                chars[x][y] = Chunk::new(self.image.view(
                    x as u32 * CHAR_WIDTH,
                    y as u32 * CHAR_HEIGHT,
                    CHAR_WIDTH,
                    CHAR_HEIGHT,
                ))
                .get_best_char();
            }
        }

        ProcessedImage::new(chars)
    }
}

pub struct ProcessedImage {
    chars: [[VGAChar; VGA_CHAR_HEIGHT]; VGA_CHAR_WIDTH],
}

impl ProcessedImage {
    pub fn new(chars: [[VGAChar; VGA_CHAR_HEIGHT]; VGA_CHAR_WIDTH]) -> Self {
        ProcessedImage { chars }
    }

    pub fn render(&self) -> Result<RgbImage, image::error::ImageError> {
        let mut image_buf = RgbImage::new(VGA_PIXEL_WIDTH, VGA_PIXEL_HEIGHT);

        for y in 0..VGA_CHAR_HEIGHT as u32 {
            for x in 0..VGA_CHAR_WIDTH as u32 {
                let image = &VGACHAR_LOOKUP[self.chars[x as usize][y as usize].lookup_index()].1;
                image_buf.copy_from(&image.to_image(), x * CHAR_WIDTH, y * CHAR_HEIGHT)?;
            }
        }

        Ok(image_buf)
    }
}
