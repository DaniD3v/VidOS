use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn extract_frames(path: &Path) -> Result<(), Box<dyn Error>> {
    fs::remove_dir_all("/tmp/VidOS")?;
    fs::create_dir("/tmp/VidOS")?;

    Command::new("ffmpeg")
        .arg("-i")
        .arg(path)
        .arg("/tmp/VidOS/%7d.png")
        .output()?;

    Ok(())
}
