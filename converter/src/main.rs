#![feature(lazy_cell)]

use std::path::Path;
use std::time::Instant;

use image_::Image;

mod char;
mod constants;
mod image_;

pub fn main() {
    let time = Instant::now();

    let image = Image::new(&Path::new("./videos/in/incredible.jpg")).unwrap();
    image
        .process_image()
        .render()
        .unwrap()
        .save("./videos/out/result.png")
        .unwrap();

    println!("{:?} elapsed.", time.elapsed());
}
