use std::collections::HashSet;
use std::time::Instant;

use image::{GenericImageView, Rgb, RgbImage};

use crate::constants::*;
use crate::image::Chunk;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

    pub fn generate_lookup_table() -> Box<[(Self, Chunk)]> {
        let now = Instant::now();
        let mut table = Vec::with_capacity(POSSIBLE_CHARS);

        for char in 0..=255 {
            for foreground in 0..FOREGROUND.len() as u8 {
                for background in 0..BACKGROUND.len() as u8 {
                    let char = Self::new(char, foreground, background);
                    let render = char.render();
                    let chunk = Chunk::new(render.view(0, 0, CHAR_WIDTH, CHAR_HEIGHT));

                    table.push((char, chunk.clone()));
                }
            }
        }

        println!("Generating lookup table took {:?}", now.elapsed());
        println!("Lookup Table Entries: {}", table.len());
        table.into_boxed_slice()
    }

    // TODO dedup
    pub fn generate_bitmaps() -> Box<
        [(
            u32,
            u32,
            [[bool; CHAR_WIDTH as usize]; CHAR_HEIGHT as usize],
        ); 236],
    > {
        let mut maps = HashSet::new();

        for char in 0..=255u32 {
            let x = char % 32; // there's 32 chars each row
            let y = char / 32;

            let bitmap = CODEPAGE_737
                .view(x * CHAR_WIDTH, y * CHAR_HEIGHT, CHAR_WIDTH, CHAR_HEIGHT)
                .to_image();

            let mut map = [[false; CHAR_WIDTH as usize]; CHAR_HEIGHT as usize];

            let mut fg_count = 0;
            let mut bg_count = 0;

            for x in 0..CHAR_WIDTH {
                for y in 0..CHAR_HEIGHT {
                    let flipped = match bitmap.get_pixel(x, y).0 {
                        [170, 170, 170] => {
                            fg_count += 1;
                            true
                        }
                        [0, 0, 0] => {
                            bg_count += 1;
                            false
                        }
                        _ => panic!(),
                    };

                    map[y as usize][x as usize] = flipped;
                }
            }

            maps.insert((fg_count, bg_count, map));
        }

        let maps = maps.into_iter().collect::<Vec<_>>();

        maps.into_boxed_slice().try_into().unwrap()
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

    fn render(&self) -> RgbImage {
        let mut image = RgbImage::new(CHAR_WIDTH, CHAR_HEIGHT);
        let bitmap = self.get_bitmap();

        for x in 0..CHAR_WIDTH {
            for y in 0..CHAR_HEIGHT {
                let color = match bitmap.get_pixel(x, y).0 {
                    [170, 170, 170] => FOREGROUND[self.foreground as usize],
                    [0, 0, 0] => BACKGROUND[self.background as usize],
                    _ => panic!(),
                };

                image.put_pixel(x, y, Rgb(color))
            }
        }

        image
    }

    // TODO remove
    fn get_bitmap(&self) -> RgbImage {
        let x = self.char as u32 % 32; // there's 32 chars each row
        let y = self.char as u32 / 32;

        CODEPAGE_737
            .view(x * CHAR_WIDTH, y * CHAR_HEIGHT, CHAR_WIDTH, CHAR_HEIGHT)
            .to_image()
    }
}
