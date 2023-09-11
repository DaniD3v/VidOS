#![feature(lazy_cell)]

use std::path::PathBuf;

use image_::Image;

mod char;
mod constants;
mod image_;

pub fn main() {
    let image = Image::new(&PathBuf::from("./videos/in/test2.jpg")).unwrap();
    image.process_image().render().unwrap().save("./videos/out/resulllllt.png");
}
