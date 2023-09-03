use crate::constants::{BACKGROUND, CHARACTERS, CHAR_HEIGHT, CHAR_WIDTH, FOREGROUND};
use image::{Rgb, RgbImage};

pub struct Char {
    bitmap: &'static [u16; 9],
    foreground_color: [u8; 3],
    background_color: [u8; 3],
}

impl Char {
    pub fn new(char: usize, foreground: usize, background: usize) -> Self {
        Char {
            bitmap: &CHARACTERS[char],
            foreground_color: FOREGROUND[foreground],
            background_color: BACKGROUND[background],
        }
    }

    pub fn get_color(&self, x: u8, y: u8) -> Rgb<u8> {
        match self.bitmap[x as usize] & (1 << y) != 0 {
            true => Rgb::from(self.foreground_color),
            false => Rgb::from(self.background_color),
        }
    }

    pub fn to_image(&self) -> RgbImage {
        let mut image = RgbImage::new(CHAR_WIDTH, CHAR_HEIGHT);

        for x in 0..CHAR_WIDTH {
            for y in 0..CHAR_HEIGHT {
                image.put_pixel(x, y, self.get_color(x as u8, y as u8));
            }
        }

        image
    }
}

pub struct CharID {
    id: u16,
}

impl CharID {
    pub fn new() -> Self {
        CharID { id: 0 }
    }
}

impl Iterator for CharID {
    type Item = Char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.id == u16::MAX {
            return None;
        }

        let char = (self.id % 256) as usize;
        let foreground = ((self.id / 256) % 16) as usize;
        let background = ((self.id / 4096) % 8) as usize;

        self.id += 1;

        Some(Char::new(char, foreground, background))
    }
}
