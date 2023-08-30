use sha2::{Digest, Sha256};
use std::ffi::OsString;

use std::fs::{read_dir, DirEntry, File};
use std::io;
use std::io::Error;

fn main() {
    let _files = get_unconverted_files();
}

fn get_unconverted_files() -> Result<Vec<DirEntry>, Error> {
    let out_names: Vec<_> = read_dir("./videos/out")?
        .map(|entry| entry.map(|x| x.file_name()))
        .collect::<Result<_, _>>()?;

    Ok(read_dir("./videos/in")?
        .map(|file| {
            file.map(|file| {
                let mut hasher = Sha256::new();
                io::copy(&mut File::open(file.path()).ok()?, &mut hasher).ok()?;
                let name = format!("{:?}.data", hasher.finalize());

                println!("{name}");

                (!out_names.contains(&OsString::from(name))).then_some(file)
            })
                .ok()
                .flatten()
        })
        .collect::<Option<Vec<_>>>()
        .unwrap_or_default())
}