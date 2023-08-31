mod converter;

use sha2::{Digest, Sha256};

use std::ffi::OsString;
use std::fs::{read_dir, DirEntry, File};
use std::io;
use std::ptr::hash;

fn main() {
    let _files = get_unconverted_files();
}

fn hashed_filename(file: &DirEntry) -> Result<String, io::Error> {
    let mut hasher = Sha256::new();
    io::copy(&mut File::open(file.path()), &mut hasher)?;
    Ok(format!("{:x}.data", hasher.finalize()))
}

fn get_unconverted_files() -> Result<Vec<DirEntry>, io::Error> {
    let out_names: Vec<String> = read_dir("./videos/out")?
        .map(|entry| Ok(entry?.file_name()))
        .collect::<Result<_, _>>()?;

    Ok(read_dir("./videos/in")?
        .map(|file| (!out_names.contains(&hashed_filename(&file?)?).then(file?)))
        .collect())
}
