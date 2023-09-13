#![feature(lazy_cell)]
#![feature(iter_array_chunks)]
#![feature(let_chains)]

use std::path::Path;
use std::time::Instant;

use image_::Image;

mod char;
mod chunk;
mod constants;
mod image_;

pub fn main() {
    let time = Instant::now();

    let image = Image::new(Path::new("./videos/in/incredible.jpg")).unwrap();
    image
        .process_image()
        .render()
        .unwrap()
        .save("./videos/out/result.png")
        .unwrap();

    println!("{:?} elapsed.", time.elapsed());
}
