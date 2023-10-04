#![feature(lazy_cell)]
#![feature(path_file_prefix)]

pub mod constants;
pub mod image;
pub mod char;

use std::error::Error;
use std::path::PathBuf;
use std::fs::read_dir;
use std::time::Instant;

use crate::image::Image;

pub fn main() -> Result<(), Box<dyn Error>> {
    for file in read_dir("./examples/images/in")? {
        let path = file?.path();
        let image = Image::new(&path)?;

        println!("Processing image {:?} -> {}", path, output_path(&path));

        let time = Instant::now();

        image.process_image()
            .render()?
            .save(output_path(&path))?;

        println!("{:?} elapsed.", time.elapsed());
    }

    Ok(())
}

fn output_path(path: &PathBuf) -> String {
    let images_folder = path.parent().unwrap().parent().unwrap().to_str().unwrap();
    let filename = path.file_prefix().unwrap().to_str().unwrap();

    format!("{}/out/{}.png", images_folder, filename)
}
