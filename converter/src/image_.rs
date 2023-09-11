use std::error::Error;
use std::path::PathBuf;

use image::{GenericImage, GenericImageView, RgbImage, SubImage};
use image::imageops::{FilterType, resize};
use image::io::Reader;

use crate::char::VGAChar;
use crate::constants::*;

pub struct Image {
    pub image: RgbImage,
}

impl Image {
    pub fn new(path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let image = Reader::open(path)?.decode()?.into_rgb8();

        // triangle (#2) is ~ 0.8 times slower than Nearest (#1) but but way better than Nearest and faster than the other ones.
        Ok(Image {
            image: resize(
                &image,
                VGA_PIXEL_WIDTH,
                VGA_PIXEL_HEIGHT,
                FilterType::Triangle,
            )
            .into(),
        })
    }

    pub fn process_image(&self) -> ProcessedImage {
        let mut chars = [[VGAChar::new(0, 0, 0); VGA_CHAR_HEIGHT]; VGA_CHAR_WIDTH];

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
                image_buf.copy_from(image, x * CHAR_WIDTH, y * CHAR_HEIGHT)?;
            }
        }

        Ok(image_buf)
    }
}
struct Chunk<'a> {
    image: SubImage<&'a RgbImage>,
}

impl<'a> Chunk<'a> {
    pub fn new(image: SubImage<&'a RgbImage>) -> Self {
        Chunk { image }
    }

    pub fn get_best_char(&self) -> VGAChar {
        let mut min_difference = u32::MAX;
        let mut best_char = &VGAChar::new(0, 0, 0);

        for possibility in 0..POSSIBLE_CHARS as u32 {
            let difference = self.difference(possibility, min_difference);

            match difference {
                Some(difference) => {
                    min_difference = difference;
                    best_char = &VGACHAR_LOOKUP[possibility as usize].0;
                }
                None => (),
            }
        }

        best_char.clone()
    }

    fn difference(&self, char: u32, stop: u32) -> Option<u32> {
        let other = &VGACHAR_LOOKUP[char as usize].1;

        let bounds = self.image.bounds();
        let bounds = (bounds.2, bounds.3);
        assert_eq!(bounds, (other.bounds().2, other.bounds().3));

        let mut difference = 0u32;

        for y in 0..bounds.1 {
            for x in 0..bounds.0 {
                let pixel = self.image.get_pixel(x, y);
                let other_pixel = other.get_pixel(x, y);

                for color in 0..3 {
                    difference += (pixel[color] as i32 - other_pixel[color] as i32).unsigned_abs();
                }
            }
            if difference > stop {
                return None;
            }
        }

        Some(difference)
    }
}
