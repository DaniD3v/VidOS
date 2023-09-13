#[cfg(target_arch = "x86")]
use std::arch::x86::{__m256i, _mm256_sad_epu8};
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{__m256i, _mm256_sad_epu8};

use crate::char::VGAChar;
use crate::constants::{CharGrid, CHAR_HEIGHT, CHAR_WIDTH, POSSIBLE_CHARS, VGACHAR_LOOKUP};
use image::{GenericImageView, RgbImage, SubImage};

#[derive(Debug)]
pub struct Chunk {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pixels: [__m256i; 14],

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    pixels: [[Rgb<u8>; CHAR_HEIGHT as usize]; CHAR_WIDTH as usize],
}

impl Chunk {
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    pub fn new(image: SubImage<&RgbImage>) -> Self {
        let pixels = image
            .pixels()
            .map(|x| x.2)
            .array_chunks()
            .array_chunks()
            .next()
            .unwrap();

        Chunk { pixels }
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub fn new(image: SubImage<&RgbImage>) -> Self {
        let pixels = image
            .pixels()
            .map(|x| x.2)
            .array_chunks::<16>()
            .array_chunks::<9>()
            .next()
            .unwrap();

        // Append a single row. So we can convert to __256i
        let pixels: [u128; 27] = unsafe { std::mem::transmute(pixels) };
        let mut pixels = pixels.to_vec();
        pixels.push(0u128);
        let pixels: [u128; 28] = pixels.try_into().unwrap();
        let pixels = unsafe { std::mem::transmute(pixels) };

        Self { pixels }
    }

    pub fn get_best_char(&self) -> VGAChar {
        let mut min_difference = u32::MAX;
        let mut best_char = &VGAChar::new(0, 0, 0);

        for possibility in 0..POSSIBLE_CHARS as u32 {
            let difference = self.difference(possibility, min_difference);

            if let Some(difference) = difference && difference < min_difference {
                min_difference = difference;
                best_char = &VGACHAR_LOOKUP[possibility as usize].0;
            }
        }

        *best_char
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    fn difference(&self, char: u32, stop: u32) -> Option<u32> {
        let other = &VGACHAR_LOOKUP[char as usize].1;

        let mut difference = 0u32;

        for y in 0..CHAR_HEIGHT as usize {
            for x in 0..CHAR_WIDTH as usize {
                let pixel = self.pixels[x][y];
                let other_pixel = other[x][y];

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

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn difference(&self, char: u32, _: u32) -> Option<u32> {
        let other = &VGACHAR_LOOKUP[char as usize].1;

        let difference = (0..self.pixels.len())
            .map(|i| {
                let x: [u64; 4] = unsafe {
                    std::mem::transmute(_mm256_sad_epu8(self.pixels[i], other.pixels[i]))
                };
                (x[0] + x[1] + x[2] + x[3]) as u32
            })
            .sum();

        Some(difference)
    }

    // FIXME: to_image can be removed when we dont output single files as pngs anymore
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    pub fn to_image(pixels: &CharGrid) -> RgbImage {
        let mut image = RgbImage::new(CHAR_WIDTH, CHAR_HEIGHT);
        for x in 0..CHAR_WIDTH {
            for y in 0..CHAR_HEIGHT {
                image.put_pixel(x, y, pixels[x as usize][y as usize])
            }
        }
        image
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub fn to_image(&self) -> RgbImage {
        let pixels: &CharGrid = unsafe { std::mem::transmute(&self.pixels) };

        let mut image = RgbImage::new(CHAR_WIDTH, CHAR_HEIGHT);
        for x in 0..CHAR_WIDTH {
            for y in 0..CHAR_HEIGHT {
                image.put_pixel(x, y, pixels[x as usize][y as usize])
            }
        }
        image
    }
}
