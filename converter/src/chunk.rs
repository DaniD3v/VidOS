use image::{GenericImageView, RgbImage, SubImage};
use std::mem::MaybeUninit;

use crate::char::{Char, CharID};
use crate::constants::{CHAR_HEIGHT, CHAR_WIDTH, VGA_HEIGHT, VGA_WIDTH};

pub struct Chunk<'a> {
    pub image: SubImage<&'a RgbImage>,
}

impl Chunk<'_> {
    pub fn get_best_char(&self) -> Char {
        // TODO: performance

        let mut min_difference = u32::MAX;
        let mut best_char = Char::new(0, 0, 0);

        let pixels = self
            .image
            .pixels()
            .map(|(_, _, color)| color)
            .collect::<Vec<_>>();

        for possibility in CharID::new() {
            let difference = self.difference(&possibility, min_difference)

            match difference {
                Some => {
                    min_difference = difference;
                    best_char = possibility;
                }
                None => ()
            }
        }

        best_char
    }

    fn difference(&self, other: &Char, stop: u32) -> Option<u32> {
        let mut difference = 0u32;

        for y in 0..CHAR_HEIGHT {
            for x in 0..CHAR_WIDTH {
                let pixel = self.image.get_pixel(x, y);
                let other_pixel = other.get_color(x as u8, y as u8);

                for color in 0..3 {
                    difference += (pixel[color] as i32 - other_pixel[color] as i32).unsigned_abs();
                }

                if difference > stop: return None
            }
        }

        Some(difference)
    }
}

pub fn chunk_up(image: RgbImage) -> Box<[[Char; VGA_WIDTH as usize]; VGA_HEIGHT as usize]> {
    let width = image.width();
    let height = image.height();
    assert_eq!(width, CHAR_WIDTH * VGA_WIDTH);
    assert_eq!(height, CHAR_HEIGHT * VGA_HEIGHT);

    // TODO: make this not awful
    const UNINIT_CHAR: MaybeUninit<Char> = MaybeUninit::uninit();
    const UNINIT_ROW: MaybeUninit<[Char; VGA_WIDTH as usize]> = MaybeUninit::uninit();

    let mut chars = Box::new([UNINIT_ROW; VGA_HEIGHT as usize]);
    let mut row = [UNINIT_CHAR; VGA_WIDTH as usize];
    let mut column = 0;
    let mut row_index = 0;
    for y in (0..height).step_by(CHAR_HEIGHT as usize) {
        for x in (0..width).step_by(CHAR_WIDTH as usize) {
            let chunk = Chunk {
                image: image.view(x, y, CHAR_WIDTH, CHAR_HEIGHT),
            };
            let best_char = chunk.get_best_char();
            row[column] = MaybeUninit::new(best_char);
            column += 1;
        }
        column = 0;
        let row = std::mem::replace(&mut row, [UNINIT_CHAR; VGA_WIDTH as usize]);
        chars[row_index] = MaybeUninit::new(row.map(|x| unsafe { x.assume_init() }));

        row_index += 1;
        println!("{row_index}");
    }

    Box::new(chars.map(|row| unsafe { row.assume_init() }))
}
