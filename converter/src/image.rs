use std::arch::x86_64::{__m128i, _mm_sad_epu8};
use std::error::Error;
use std::path::PathBuf;

use image::imageops::{resize, FilterType};
use image::io::Reader;
use image::{GenericImage, GenericImageView, RgbImage, SubImage};

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
            ),
        })
    }

    // no threading here because doing it at the thread/frame stage is much more efficient
    pub fn process_image(&self) -> ProcessedImage {
        let mut chars = [[VGAChar::uninit(); VGA_CHAR_WIDTH]; VGA_CHAR_HEIGHT];

        for x in 0..VGA_CHAR_WIDTH {
            for (y, row) in chars.iter_mut().enumerate().take(VGA_CHAR_HEIGHT) {
                row[x] = Chunk::new(self.image.view(
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

impl From<RgbImage> for Image {
    fn from(image: RgbImage) -> Self {
        Self {
            image: resize(
                &image,
                VGA_PIXEL_WIDTH,
                VGA_PIXEL_HEIGHT,
                FilterType::Triangle,
            ),
        }
    }
}

pub struct ProcessedImage {
    chars: [[VGAChar; VGA_CHAR_WIDTH]; VGA_CHAR_HEIGHT],
}

impl ProcessedImage {
    pub fn new(chars: [[VGAChar; VGA_CHAR_WIDTH]; VGA_CHAR_HEIGHT]) -> Self {
        ProcessedImage { chars }
    }

    pub fn render(&self) -> Result<RgbImage, image::error::ImageError> {
        let mut image_buf = RgbImage::new(VGA_PIXEL_WIDTH, VGA_PIXEL_HEIGHT);

        for x in 0..VGA_CHAR_WIDTH as u32 {
            for y in 0..VGA_CHAR_HEIGHT as u32 {
                let chunk = &VGACHAR_LOOKUP[self.chars[y as usize][x as usize].lookup_index()].1;
                // TODO: this sucks
                let chunk = chunk.image.flatten().to_vec();

                let image = RgbImage::from_raw(CHAR_WIDTH, CHAR_HEIGHT, chunk.flatten().to_vec());
                let image = image.unwrap();
                image_buf.copy_from(&image, x * CHAR_WIDTH, y * CHAR_HEIGHT)?;
            }
        }

        Ok(image_buf)
    }

    pub fn serialize(&self) -> [u8; VGA_WORD_SIZE * 2] {
        unsafe {
            std::mem::transmute::<[u16; VGA_WORD_SIZE], _>(
                self.chars
                    .iter()
                    .flatten()
                    .map(VGAChar::vga_format)
                    .collect::<Vec<u16>>()
                    .try_into()
                    .unwrap(),
            )
        }
    }
}

type Grid = [[[u8; 3]; CHAR_WIDTH as usize]; CHAR_HEIGHT as usize];

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "simd_if_available"
))]
pub fn total_diff(left: &Grid, right: &Grid) -> u32 {
    let left: &[__m128i; 27] = unsafe { std::mem::transmute(left) };
    let right: &[__m128i; 27] = unsafe { std::mem::transmute(right) };

    let mut total = 0;

    for i in 0..27 {
        let left = left[i];
        let right = right[i];

        let sums = unsafe { _mm_sad_epu8(left, right) };

        let sums: [u64; 2] = unsafe { std::mem::transmute(sums) };
        for i in sums {
            total += i as u32;
        }
    }

    total
}

#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "simd_if_available"
)))]
pub fn total_diff(left: &Grid, right: &Grid) -> u32 {
    let mut difference = 0u32;

    for y in 0..CHAR_HEIGHT {
        for x in 0..CHAR_WIDTH {
            let pixel = left[y as usize][x as usize];
            let other_pixel = right[y as usize][x as usize];

            for color in 0..3 {
                difference += (pixel[color] as i32 - other_pixel[color] as i32).unsigned_abs();
            }
        }
    }

    difference
}

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
pub struct Chunk {
    // TODO impl Hash -> cache frequent chunks
    image: Box<Grid>,
}

impl Chunk {
    pub fn new(image: SubImage<&RgbImage>) -> Self {
        let pixels: Vec<_> = image.pixels().map(|(_, _, c)| c.0).collect();
        let pixels: &[[[u8; 3]; CHAR_WIDTH as usize]] = pixels.as_chunks().0;
        Chunk {
            image: pixels.to_vec().into_boxed_slice().try_into().unwrap(), // TODO this sucks
        }
    }

    pub fn get_best_char(&self) -> VGAChar {
        if let Some(char) = FIXED_CACHE.get(self) {
            return *char;
        }

        if let Some(char) = DYNAMIC_CACHE.lock().unwrap().get(self) {
            return *char;
        }

        let mut min_diff = u32::MAX;
        let mut best_char = None;

        for (char_i, (mut fg_count, mut bg_count, char)) in BITMAPS.iter().enumerate() {
            let (fg_sum, bg_sum) = sums(&self.image, char);

            if bg_count == 0 {
                assert_eq!(bg_sum, [0, 0, 0]);
                bg_count = 1;
            }
            if fg_count == 0 {
                assert_eq!(fg_sum, [0, 0, 0]);
                fg_count = 1;
            }

            let fg_avg = fg_sum.map(|x| (x as f64 / fg_count as f64).round() as u8);
            let bg_avg = bg_sum.map(|x| (x as f64 / bg_count as f64).round() as u8);

            let fg = most_similar(fg_avg, &FOREGROUND);
            let bg = most_similar(bg_avg, &BACKGROUND);

            let char = VGAChar::new(char_i as u8, fg as u8, bg as u8);
            let new_char = &VGACHAR_LOOKUP[char.lookup_index()].1.image;
            let diff = total_diff(&self.image, new_char);

            if diff < min_diff {
                min_diff = diff;
                best_char = Some(char);
            }
        }

        let best_char = best_char.expect("Is CHARS empty?");
        DYNAMIC_CACHE
            .lock()
            .unwrap()
            .insert(self.clone(), best_char);

        best_char
    }
}

fn most_similar(to: [u8; 3], from: &[[u8; 3]]) -> usize {
    from.iter()
        .enumerate()
        .min_by_key(|(_, v)| {
            to[0].abs_diff(v[0]) as u32 + to[1].abs_diff(v[1]) as u32 + to[2].abs_diff(v[2]) as u32
        })
        .map(|(i, _)| i)
        .unwrap()
}

pub fn sums(
    colored: &Grid,
    bitmap: &[[bool; CHAR_WIDTH as usize]; CHAR_HEIGHT as usize],
) -> ([u32; 3], [u32; 3]) {
    let mut fg_sum = [0; 3];
    let mut bg_sum = [0; 3];

    for y in 0..CHAR_HEIGHT as usize {
        for x in 0..CHAR_WIDTH as usize {
            let color = colored[y][x];
            if bitmap[y][x] {
                fg_sum[0] += color[0] as u32;
                fg_sum[1] += color[1] as u32;
                fg_sum[2] += color[2] as u32;
            } else {
                bg_sum[0] += color[0] as u32;
                bg_sum[1] += color[1] as u32;
                bg_sum[2] += color[2] as u32;
            }
        }
    }

    (fg_sum, bg_sum)
}
