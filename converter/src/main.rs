#![feature(lazy_cell)]

use std::path::PathBuf;
use std::time::Instant;

use image_::Image;

mod char;
mod constants;
mod image_;

pub fn main() {
    let time = Instant::now();

    let image = Image::new(&PathBuf::from("./videos/in/test1.png")).unwrap();
    image.process_image().render().unwrap().save("./videos/out/result.png").unwrap();

    println!("{:?} elapsed.", time.elapsed());
}
