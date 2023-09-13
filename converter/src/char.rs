use image::{GenericImageView, Rgb, RgbImage};
use std::time::Instant;

use crate::chunk::Chunk;
use crate::constants::*;

#[derive(Debug, Clone, Copy)]
pub struct VGAChar {
    char: u8,
    foreground: u8,
    background: u8,
}

impl VGAChar {
    pub fn new(char: u8, foreground: u8, background: u8) -> Self {
        if foreground >= FOREGROUND.len() as u8 || background >= BACKGROUND.len() as u8 {
            panic!("Invalid color");
        }

        Self {
            char,
            foreground,
            background,
        }
    }

    pub fn uninit() -> Self {
        Self {
            char: 0,
            foreground: 0,
            background: 0,
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    pub fn generate_lookup_table() -> Box<[(Self, Chunk); POSSIBLE_CHARS]> {
        let now = SystemTime::now();
        let mut table = Vec::with_capacity(POSSIBLE_CHARS);

        for char in 0..=255 {
            for foreground in 0..FOREGROUND.len() as u8 {
                for background in 0..BACKGROUND.len() as u8 {
                    let char = Self::new(char, foreground, background);
                    let render = Chunk {
                        pixels: char.render(),
                    };

                    table.push((char, render))
                }
            }
        }

        println!("Generating lookup table took {:?}", now.elapsed());
        table.into_boxed_slice().try_into().unwrap()
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub fn generate_lookup_table() -> Box<[(Self, Chunk); POSSIBLE_CHARS]> {
        let now = Instant::now();
        let mut table = Vec::with_capacity(POSSIBLE_CHARS);

        for char in 0..=255 {
            for foreground in 0..FOREGROUND.len() as u8 {
                for background in 0..BACKGROUND.len() as u8 {
                    let char = Self::new(char, foreground, background);
                    let render = char.render();

                    // Append a single row. So we can convert to __256i
                    let render: [u128; 27] = unsafe { std::mem::transmute(render) };
                    let mut render = render.to_vec();
                    render.push(0u128);
                    let render: [u128; 28] = render.try_into().unwrap();

                    let render = unsafe { std::mem::transmute(render) };
                    table.push((char, render));
                }
            }
        }

        println!("Generating lookup table took {:?}", now.elapsed());
        table.into_boxed_slice().try_into().unwrap()
    }

    #[allow(dead_code)] // TODO
    pub fn vga_format(&self) -> u16 {
        self.char as u16 | ((self.foreground as u16) << 8) | ((self.background as u16) << 12)
    }

    pub fn lookup_index(&self) -> usize {
        self.char as usize * FOREGROUND.len() * BACKGROUND.len()
            + self.foreground as usize * BACKGROUND.len()
            + self.background as usize
    }

    fn render(&self) -> CharGrid {
        let mut image = [[Rgb::from([0, 0, 0]); CHAR_HEIGHT as usize]; CHAR_WIDTH as usize];
        let bitmap = self.get_bitmap();

        for x in 0..CHAR_WIDTH {
            for y in 0..CHAR_HEIGHT {
                let color = match bitmap.get_pixel(x, y).0 {
                    [170, 170, 170] => FOREGROUND[self.foreground as usize],
                    [0, 0, 0] => BACKGROUND[self.background as usize],
                    _ => panic!(),
                };

                image[x as usize][y as usize] = Rgb(color);
            }
        }

        image
    }

    fn get_bitmap(&self) -> RgbImage {
        let x = self.char as u32 % 32; // there's 32 chars each row
        let y = self.char as u32 / 32;

        CODEPAGE_737
            .view(x * CHAR_WIDTH, y * CHAR_HEIGHT, CHAR_WIDTH, CHAR_HEIGHT)
            .to_image()
    }
}
