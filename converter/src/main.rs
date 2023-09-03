#![feature(const_trait_impl)]
#![feature(array_chunks)]
mod char;
mod chunk;
mod constants;
mod converter;
mod other;
//mod chunker;

use sha2::{Digest, Sha256};
use std::ffi::OsString;
use std::fs::{read_dir, File};
use std::io;
use std::path::PathBuf;

use crate::constants::{CHAR_HEIGHT, CHAR_WIDTH, VGA_HEIGHT, VGA_WIDTH};
use image::io::Reader as ImageReader;
use image::{GenericImage, RgbImage};

pub fn main() {
    let image = ImageReader::open("./videos/test.png")
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();

    let chars = chunk::chunk_up(image);

    let mut img_buf = RgbImage::new(CHAR_WIDTH * VGA_WIDTH, VGA_HEIGHT * CHAR_HEIGHT);
    for (y, row) in chars.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            let image = char.to_image();
            img_buf
                .copy_from(&image, x as u32 * CHAR_WIDTH, y as u32 * CHAR_HEIGHT)
                .unwrap();
        }
    }

    img_buf.save("./videos/testing_output.png").unwrap();

    println!("Bye!");
}

fn hashed_filename(path: &PathBuf) -> Result<OsString, io::Error> {
    let mut hasher = Sha256::new();
    io::copy(&mut File::open(path)?, &mut hasher)?;
    Ok(format!("{:x}.data", hasher.finalize()).into())
}

pub fn get_unconverted_files() -> Result<Vec<(PathBuf, OsString)>, io::Error> {
    let output_videos: Vec<OsString> = read_dir("./videos/out")?
        .map(|entry| Ok::<_, io::Error>(entry?.file_name()))
        .collect::<Result<_, _>>()?;

    let mut input_videos = Vec::new();
    for entry in read_dir("./videos/in")? {
        let path = entry?.path();
        let hash = hashed_filename(&path)?;

        if !output_videos.contains(&hash) {
            input_videos.push((path, hash))
        }
    }

    Ok(input_videos)
}
