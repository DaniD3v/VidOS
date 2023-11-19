#![feature(lazy_cell)]
#![feature(path_file_prefix)]
#![feature(fs_try_exists)]
#![feature(slice_as_chunks)]
#![feature(slice_flatten)]

pub mod char;
pub mod constants;
pub mod image;
mod video;

use crate::char::VGAChar;
use crate::constants::DYNAMIC_CACHE;
use ::image::RgbImage;
use std::error::Error;
use std::fs;
use std::fs::read_dir;
use std::io::Write;
use std::mem::size_of;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crate::image::{Chunk, Image, ProcessedImage};
use crate::video::for_each_frame;

pub fn cache_cleaner() {
    // 1Gb
    const MAX_BYTES: usize = 1024 * 1024 * 1024;

    loop {
        let mut map = DYNAMIC_CACHE.lock().unwrap();
        let size = map.len() * size_of::<Chunk>() * size_of::<VGAChar>();

        if size > MAX_BYTES {
            // TODO: is throwing the whole cache away really the best
            println!("..... CLEARING CACHE");
            map.clear();
        }
        drop(map);

        std::thread::sleep(Duration::from_secs(60))
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    ffmpeg::init()?;

    // for file in read_dir("./examples/images/in")? {
    //     let path = file?.path();
    //     process_image(&path)?;
    // }

    for file in read_dir("./examples/videos/in")? {
        let path = file?.path();
        if let Some(name) = path.file_name() {
            //if name == "BadApple.mp4" { continue; }
            println!("Processing {name:?}.");
        }

        fn process_frame(frame: RgbImage) -> ProcessedImage {
            let start = Instant::now();
            println!("Processing Frame...");

            let ret = Image::from(frame).process_image();
            println!("Processing Frame took {:#?}", start.elapsed());
            ret
        }

        std::thread::spawn(cache_cleaner);
        for_each_frame(&path, &process_frame, &|chunk| {
            let mut bytes = vec![];
            for processed in &chunk {
                bytes.extend_from_slice(&processed.serialize());
            }

            println!("Wrote chunk of {} Frames!", chunk.len());

            fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(dir_path(&path, "ser", "bin"))
                .unwrap()
                .write_all(&bytes)
                .unwrap();
        })?;
    }

    Ok(())
}

fn process_image(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let image = Image::new(path)?;

    println!(
        "Processing image {:?} -> {}",
        path,
        dir_path(path, "ser", "bin")
    );

    let time = Instant::now();
    let image = image.process_image();
    println!("{:?} elapsed.", time.elapsed());

    image.render()?.save(dir_path(path, "out", "png"))?;

    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(dir_path(path, "ser", "bin"))?
        .write_all(&image.serialize())?;

    Ok(())
}

fn dir_path(path: &Path, directory: &str, file_type: &str) -> String {
    let images_folder = path.parent().unwrap().parent().unwrap().to_str().unwrap();
    let file_name = path.file_prefix().unwrap().to_str().unwrap();

    format!("{images_folder}/{directory}/{file_name}.{file_type}").to_string()
}
