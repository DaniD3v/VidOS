use std::collections::hash_map::DefaultHasher;
use std::ffi::OsString;

use std::fs::{read_dir, DirEntry};
use std::hash::Hasher;
use std::io::Error;

fn main() {
    let _files = get_unconverted_files();
}

fn get_unconverted_files() -> Result<Vec<DirEntry>, Error> {
    let out_names: Vec<_> = read_dir("./videos/out")?
        .map(|entry| entry.map(|x| x.file_name()))
        .collect::<Result<_, _>>()?;

    Ok(read_dir("./videos/in")?.map(|file|  {
        file.map(|file| {
            let mut hasher = DefaultHasher::new();
            let contents = std::fs::read_to_string(file.path()).ok()?;
            hasher.write(contents.as_bytes());
            let name = format!("{:x}.data", hasher.finish());

            println!("{name}");

            let name = OsString::from(name);

            (!out_names.contains(&name)).then_some(file)
        }).ok().flatten()
    }).collect::<Option<Vec<_>>>().unwrap_or_default())
}