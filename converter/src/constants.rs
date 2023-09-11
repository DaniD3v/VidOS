use std::io::Cursor;
use std::sync::LazyLock;

use image::ImageFormat::Png;
use image::io::Reader;
use image::RgbImage;

use crate::char::VGAChar;

pub const CHAR_WIDTH: u32 = 9;
pub const CHAR_HEIGHT: u32 = 16;
pub const POSSIBLE_CHARS: usize = 256 * FOREGROUND.len() * BACKGROUND.len();

pub const VGA_CHAR_WIDTH: usize = 80;
pub const VGA_CHAR_HEIGHT: usize = 25;

pub const VGA_PIXEL_WIDTH: u32 = VGA_CHAR_WIDTH as u32 * CHAR_WIDTH;
pub const VGA_PIXEL_HEIGHT: u32 = VGA_CHAR_HEIGHT as u32 * CHAR_HEIGHT;

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

pub static VGACHAR_LOOKUP: LazyLock<[(VGAChar, RgbImage); POSSIBLE_CHARS]> = LazyLock::new(VGAChar::generate_lookup_table);
