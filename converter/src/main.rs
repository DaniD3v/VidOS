#![feature(lazy_cell)]
#![feature(path_file_prefix)]

pub mod constants;
pub mod image;
pub mod char;

use std::error::Error;
use std::time::Instant;
use std::fs::read_dir;
use std::path::Path;
use std::io::Write;
use std::fs;

use crate::image::Image;

pub fn main() -> Result<(), Box<dyn Error>> {
    for file in read_dir("./examples/images/in")? {
        let path = file?.path();
        let image = Image::new(&path)?;

        println!("Processing image {:?} -> {}", path, dir_path(&path, "out", "png"));

        let time = Instant::now();
        let image = image.process_image();
        println!("{:?} elapsed.", time.elapsed());

        image
            .render()?
            .save(dir_path(&path, "out", "png"))?;

        fs::OpenOptions::new()
            .write(true).create(true)
            .open(dir_path(&path, "ser", "bin"))?
            .write_all(&image.serialize())?;
    }

    Ok(())
}

fn dir_path(path: &Path, directory: &str, file_type: &str) -> String {
    let images_folder = path.parent().unwrap().parent().unwrap().to_str().unwrap();
    let file_name = path.file_prefix().unwrap().to_str().unwrap();

    format!("{images_folder}/{directory}/{file_name}.{file_type}").to_string()
}
