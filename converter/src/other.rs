use std::error::Error;
use image::io::Reader as ImageReader;
use image::{GenericImageView, RgbImage, SubImage};
use std::io;

use crate::constants::{CHARS_PER_LINE, CHAR_HEIGHT, CHAR_WIDTH};

fn generate_font() -> Result<(), Box<dyn Error>> {
    let codepage = ImageReader::open("./other/Codepage-737.png")?
        .decode()?
        .to_rgb8();
    let mut chars: [[u16; 9]; 256] = [[0; 9]; 256];

    for index in 0..256 {
        let (x, y) = (index % CHARS_PER_LINE, index / CHARS_PER_LINE);
        let view = &codepage.view(x * CHAR_WIDTH, y * CHAR_HEIGHT, CHAR_WIDTH, CHAR_HEIGHT);

        chars[index as usize] = generate_bitmap(view);
    }

    println!("pub const CHARS: [[u16; 9]; 256] = [");
    for c in chars {
        println!("    {:?},", c);
    }
    println!("];");

    Ok(())
}

fn generate_bitmap(image: &SubImage<&RgbImage>) -> [u16; 9] {
    let mut bitmap: [u16; 9] = [0; 9];

    for x in 0..CHAR_WIDTH {
        for y in 0..CHAR_HEIGHT {
            match image.get_pixel(x, y).0 {
                [170, 170, 170] => bitmap[x as usize] |= 1 << y, // 170 not 255 because the original image was generated with the VGA palette
                [0, 0, 0] => (),
                _ => {
                    image.to_image().save("./other/error.png").unwrap();
                    panic!("Invalid color: {:?}", image.get_pixel(x, y).0);
                }
            };
        }
    }

    bitmap
}
