use crate::char::{Char, CharID};
use crate::constants::{CHAR_HEIGHT, CHAR_WIDTH, VGA_HEIGHT, VGA_WIDTH};
use image::{GenericImageView, Rgb, RgbImage, SubImage};

pub struct Chunk<'a> {
    pub image: SubImage<&'a RgbImage>,
}

impl Chunk<'_> {
    pub fn get_best_char(&self) -> Char {
        let mut min_difference = u32::MAX;
        let mut best_char = Char::new(0, 0, 0);

        let pixels: [Rgb<u8>; 16 * 9] = self
            .image
            .pixels()
            .map(|(_, _, color)| color)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        'possibility: for possibility in CharID::new() {
            let mut difference = 0u32;

            for y in 0..CHAR_HEIGHT {
                for x in 0..CHAR_WIDTH {
                    let pixel = pixels[(x + y * CHAR_WIDTH) as usize];
                    let other_pixel = possibility.get_color(x as u8, y as u8);

                    for color in 0..3 {
                        difference +=
                            (pixel[color] as i32 - other_pixel[color] as i32).unsigned_abs();
                    }
                }
                if difference > min_difference {
                    continue 'possibility;
                }
            }

            if difference < min_difference {
                min_difference = difference;
                best_char = possibility;
            }
        }

        best_char
    }

    fn difference(&self, other: Char) -> u32 {
        let mut difference = 0u32;

        for y in 0..CHAR_HEIGHT {
            for x in 0..CHAR_WIDTH {
                let pixel = self.image.get_pixel(x, y);
                let other_pixel = other.get_color(x as u8, y as u8);

                for color in 0..3 {
                    difference += (pixel[color] as i32 - other_pixel[color] as i32).unsigned_abs();
                }
            }
        }

        difference
    }
}
pub struct ChunkIter {
    image: RgbImage,
    column: u32,
    row: u32,
}
impl ChunkIter {
    pub fn new(image: RgbImage) -> ChunkIter {
        assert_eq!(image.width(), CHAR_WIDTH * VGA_WIDTH);
        assert_eq!(image.height(), CHAR_HEIGHT * VGA_HEIGHT);

        Self {
            image,
            column: 0,
            row: 0,
        }
    }
}

impl Iterator for ChunkIter {
    type Item = (u32, u32, Char);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == VGA_WIDTH * CHAR_WIDTH {
            self.column += CHAR_HEIGHT;

            if self.column == VGA_HEIGHT * CHAR_HEIGHT {
                return None;
            }

            self.row = 0;
        }

        let chunk = Chunk {
            image: SubImage::new(&self.image, self.row, self.column, CHAR_WIDTH, CHAR_HEIGHT),
        };

        let ret = Some((self.row, self.column, chunk.get_best_char()));
        self.row += CHAR_WIDTH;
        ret
    }
}
