#![feature(const_trait_impl)]

mod char;
mod constants;
mod converter;
mod other;
mod chunk;

use sha2::{Digest, Sha256};
use std::ffi::OsString;
use std::fs::{read_dir, File};
use std::io;
use std::path::PathBuf;

use image::io::Reader as ImageReader;
use image::GenericImageView;
use crate::chunk::Chunk;

fn main() {
    let image = ImageReader::open("./other/test.jpg").unwrap().decode().unwrap().to_rgb8();
    let view = image.view(0, 0, image.width(), image.height());

    let chunk = Chunk { image: view };
    let char = chunk.get_best_char();
    char.to_image().save("./other/best_char.png").unwrap();

    let test_char = char::Char::new(1, 0, 7);
    test_char.to_image().save("./other/test_char.png").unwrap();
}

fn hashed_filename(path: &PathBuf) -> Result<OsString, io::Error> {
    let mut hasher = Sha256::new();
    io::copy(&mut File::open(path)?, &mut hasher)?;
    Ok(format!("{:x}.data", hasher.finalize()).into())
}

fn get_unconverted_files() -> Result<Vec<(PathBuf, OsString)>, io::Error> {
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
