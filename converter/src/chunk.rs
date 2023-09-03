use image::{RgbImage, SubImage};
use image::imageops::resize;

use crate::char::{CharID, Char};
use crate::constants::{CHAR_HEIGHT, CHAR_WIDTH};

pub struct Chunk<'a> {
    pub image: SubImage<&'a RgbImage>,
}

impl Chunk<'_> {

    pub fn get_best_char(&self) -> Char {
        let mut min_difference = u32::MAX;
        let mut best_char = Char::new(0, 0, 0);

        for possibility in CharID::new() {
            let difference = self.difference(&possibility);

            if difference < min_difference {
                min_difference = difference;
                best_char = possibility;
            }
        };

        best_char
    }

    fn difference(&self, other: &Char) -> u32 {
        let resized = resize(&self.image.to_image() , CHAR_WIDTH, CHAR_HEIGHT, image::imageops::FilterType::Nearest);
        let mut difference = 0u32;

        for y in 0..CHAR_HEIGHT {
            for x in 0..CHAR_WIDTH {
                let pixel = resized.get_pixel(x, y);
                let other_pixel = other.get_color(x as u8, y as u8);

                for color in 0..3 {
                    difference += (pixel[color] as i32 - other_pixel[color] as i32).abs() as u32;
                }
            }
        }

        difference
    }
}
