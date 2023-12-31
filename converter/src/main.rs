#![feature(lazy_cell)]
#![feature(path_file_prefix)]
#![feature(fs_try_exists)]

pub mod constants;
pub mod image;
pub mod char;
mod video;

use std::error::Error;
use std::time::Instant;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::io::Write;
use std::fs;

use crate::image::Image;
use crate::video::extract_frames;

pub fn main() -> Result<(), Box<dyn Error>> {
    // for file in read_dir("./examples/images/in")? {
    //     let path = file?.path();
    //     process_image(&path)?;
    // }

    for file in read_dir("./examples/videos/in")? {
        let path = file?.path();
        if let Some(name) = path.file_name() {
            if name == "BadApple.mp4" { continue; }
            println!("Processing {name:?}.");
        }

        extract_frames(&path)?;
        println!("extracted frames");

        for file in read_dir("/tmp/VidOS")? {
            let image_path = file?.path();

            if let Some(name) = image_path.file_name() { println!("Processing {name:?}."); }
            let image = Image::new(&image_path)?.process_image();

            fs::OpenOptions::new()
                .append(true).create(true)
                .open(dir_path(&path, "ser", "bin"))?
                .write_all(&image.serialize())?;
        }
    }

    Ok(())
}

fn process_image(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let image = Image::new(path)?;

    println!("Processing image {:?} -> {}", path, dir_path(path, "ser", "bin"));

    let time = Instant::now();
    let image = image.process_image();
    println!("{:?} elapsed.", time.elapsed());

    image
        .render()?
        .save(dir_path(path, "out", "png"))?;

    fs::OpenOptions::new()
        .write(true).create(true)
        .open(dir_path(path, "ser", "bin"))?
        .write_all(&image.serialize())?;

    Ok(())
}

fn dir_path(path: &Path, directory: &str, file_type: &str) -> String {
    let images_folder = path.parent().unwrap().parent().unwrap().to_str().unwrap();
    let file_name = path.file_prefix().unwrap().to_str().unwrap();

    format!("{images_folder}/{directory}/{file_name}.{file_type}").to_string()
}
