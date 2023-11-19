use std::collections::HashMap;
use std::io::Cursor;
use std::sync::{LazyLock, Mutex};

use image::io::Reader;
use image::ImageFormat::Png;
use image::RgbImage;

use crate::char::VGAChar;
use crate::image::Chunk;

pub const CHAR_WIDTH: u32 = 9;
pub const CHAR_HEIGHT: u32 = 16;
pub const POSSIBLE_CHARS: usize = 256 * FOREGROUND.len() * BACKGROUND.len();

pub const VGA_CHAR_WIDTH: usize = 80;
pub const VGA_CHAR_HEIGHT: usize = 25;

pub const VGA_PIXEL_WIDTH: u32 = VGA_CHAR_WIDTH as u32 * CHAR_WIDTH;
pub const VGA_PIXEL_HEIGHT: u32 = VGA_CHAR_HEIGHT as u32 * CHAR_HEIGHT;

pub const VGA_WORD_SIZE: usize = VGA_CHAR_WIDTH * VGA_CHAR_HEIGHT;

pub const FOREGROUND: [[u8; 3]; 16] = [
    [0, 0, 0],
    [0, 0, 170],
    [0, 170, 0],
    [0, 170, 170],
    [170, 0, 0],
    [170, 0, 170],
    [170, 85, 0],
    [170, 170, 170],
    [85, 85, 85],
    [85, 85, 255],
    [85, 255, 85],
    [85, 255, 255],
    [255, 85, 85],
    [255, 85, 255],
    [255, 255, 85],
    [255, 255, 255],
];

pub const BACKGROUND: [[u8; 3]; 8] = [
    [0, 0, 0],
    [0, 0, 170],
    [0, 170, 0],
    [0, 170, 170],
    [170, 0, 0],
    [170, 0, 170],
    [170, 85, 0],
    [170, 170, 170],
];

pub static CODEPAGE_737: LazyLock<RgbImage> = LazyLock::new(|| {
    let bytes = include_bytes!("../other/Codepage-737.png");
    Reader::with_format(Cursor::new(bytes), Png)
        .decode()
        .unwrap()
        .into_rgb8()
});

pub static VGACHAR_LOOKUP: LazyLock<Box<[(VGAChar, Chunk)]>> =
    LazyLock::new(VGAChar::generate_lookup_table);
pub static BITMAPS: LazyLock<
    Box<
        [(
            u32,
            u32,
            [[bool; CHAR_WIDTH as usize]; CHAR_HEIGHT as usize],
        ); 236],
    >,
> = LazyLock::new(VGAChar::generate_bitmaps);

pub static DYNAMIC_CACHE: LazyLock<Mutex<HashMap<Chunk, VGAChar>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub static FIXED_CACHE: LazyLock<HashMap<Chunk, VGAChar>> = LazyLock::new(|| {
    let mut map = HashMap::with_capacity(VGACHAR_LOOKUP.len());
    for (char, chunk) in VGACHAR_LOOKUP.iter() {
        map.insert(chunk.clone(), *char);
    }
    map
});

#[cfg(test)]
mod tests {
    use crate::constants::VGACHAR_LOOKUP;

    #[test]
    fn lookup_index() {
        for (i, (v, _)) in VGACHAR_LOOKUP.iter().enumerate() {
            assert_eq!(i, v.lookup_index());
        }
    }

    #[test]
    fn best_char_lookup() {
        for (i, (v, x)) in VGACHAR_LOOKUP.iter().enumerate() {
            let best = x.get_best_char();
            let best = &VGACHAR_LOOKUP[best.lookup_index()].1;
            assert_eq!(x, best);
        }
    }
}
