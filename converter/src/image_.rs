use std::error::Error;
use std::path::PathBuf;

use image::{GenericImage, GenericImageView, RgbImage, SubImage};
use image::imageops::{FilterType, resize};
use image::io::Reader;

use crate::char::VGAChar;
use crate::constants::{CHAR_HEIGHT, CHAR_WIDTH, DOWNSCALE_HEIGHT, DOWNSCALE_WIDTH, POSSIBLE_CHARS, VGA_CHAR_HEIGHT, VGA_CHAR_WIDTH, VGA_PIXEL_HEIGHT, VGA_PIXEL_WIDTH, VGACHAR_LOOKUP};

pub struct Image {
    pub image: RgbImage
}

impl Image {
    pub fn new(path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let image = Reader::open(path)?.decode()?.into_rgb8();

        // triangle (#2) is ~ 0.8 times slower than Nearest (#1) but way better and faster than the other ones.
        Ok(Image { image: resize(&image, VGA_PIXEL_WIDTH, VGA_PIXEL_HEIGHT, FilterType::Triangle).into() })
    }

    pub fn process_image(&self) -> ProcessedImage {
        let mut chars = [[VGAChar::new(0, 0, 0); VGA_CHAR_HEIGHT]; VGA_CHAR_WIDTH];

        for y in 0..VGA_CHAR_HEIGHT {
            for x in 0..VGA_CHAR_WIDTH {
                chars[x][y] = Chunk::new(self.image.view(x as u32 * CHAR_WIDTH, y as u32 * CHAR_HEIGHT, CHAR_WIDTH, CHAR_HEIGHT)).get_best_char();
            }
        }

        ProcessedImage::new(chars)
    }
}

pub struct ProcessedImage {
    chars: [[VGAChar; VGA_CHAR_HEIGHT]; VGA_CHAR_WIDTH]
}

impl ProcessedImage {
    pub fn new(chars: [[VGAChar; VGA_CHAR_HEIGHT]; VGA_CHAR_WIDTH]) -> Self {
        ProcessedImage { chars }
    }

    pub fn render(&self) -> Result<RgbImage, image::error::ImageError> {
        let mut image_buf = RgbImage::new(VGA_PIXEL_WIDTH, VGA_PIXEL_HEIGHT);

        for y in 0..VGA_CHAR_HEIGHT as u32{
            for x in 0..VGA_CHAR_WIDTH as u32{
                let image = &VGACHAR_LOOKUP[self.chars[x as usize][y as usize].lookup_index()].1;
                    image_buf.copy_from(image, x*CHAR_WIDTH, y*CHAR_HEIGHT)?;
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
        let mut best_char = VGAChar::new(0, 0, 0);

        for possibility in 0..POSSIBLE_CHARS as u32 {
            let difference = self.difference(possibility);

            if difference < min_difference {
                min_difference = difference;
                best_char = VGACHAR_LOOKUP[possibility as usize].0.clone();
            }
        }

        best_char
    }

    fn difference(&self, char: u32) -> u32 {
        let (_, render, downscale) = &VGACHAR_LOOKUP[char as usize];
        let chunk_downscale = resize(&self.image.to_image(), DOWNSCALE_WIDTH, DOWNSCALE_HEIGHT, FilterType::Nearest);

        Self::raw_difference(&chunk_downscale, &downscale)
            + Self::raw_difference(&self.image.to_image(), &render)
    }

    fn raw_difference(own: &RgbImage, other: &RgbImage) -> u32 {
        assert_eq!(own.bounds(), other.bounds());
        let mut difference = 0u32;

        for y in 0..own.bounds().3 {
            for x in 0..own.bounds().2 {
                let pixel = own.get_pixel(x, y);
                let other_pixel = other.get_pixel(x, y);

                for color in 0..3 {
                    difference += (pixel[color] as i32 - other_pixel[color] as i32).unsigned_abs();
                }
            }
        }

        difference
    }
}
