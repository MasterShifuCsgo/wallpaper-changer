use fastrand;
use std::path::PathBuf;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use windows::{
    core::PCWSTR,
    Win32::UI::WindowsAndMessaging::{
        SystemParametersInfoW, SPI_SETDESKWALLPAPER, SPIF_SENDWININICHANGE, SPIF_UPDATEINIFILE,
    },
};


// 1. puts all the jpg files in to a vector.
// 2. then chooses a random number
// 3. returns the random jpg path.
fn get_random_wallpaper(desktop_images: &PathBuf) -> Result<PathBuf, &str> {    

    //includes all the directories of the jpg files.
    let mut wallpapers: Vec<PathBuf> = Vec::new();

    // putting jpg files in vec
    for entry in desktop_images
        .read_dir()
        .expect("Failed to read directory.")
    {
        let dir = entry.expect("Entry to the path failed.");

        if let Some(ext) = dir.path().extension() {
            if ext == "jpg" {
                wallpapers.push(dir.path());
            }
        }
    }

    // error check
    if wallpapers.len() <= 0 {
        return Err("Failed to choose jpg, because no jpgs in wallpapers folder.");
    }

    // choosing random number
    let num = fastrand::usize(..wallpapers.len());    

    return Ok(wallpapers[num].clone());
}


fn set_wallpaper(path: &PathBuf) {
    let wide: Vec<u16> = OsStr::new(path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let wide_ptr = wide.as_ptr();

    unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            Some(wide_ptr as *mut _), // 👈 Cast to void pointer correctly
            SPIF_UPDATEINIFILE | SPIF_SENDWININICHANGE,
        )
        .unwrap();
    }
}
fn main() {
    let wallpapers_dir =
        PathBuf::from("C:/Users/kaspar/Desktop/Kaspar Files/Personal folder/Desktop images");

    loop {

    let wallpaper_path = get_random_wallpaper(&wallpapers_dir);

    match wallpaper_path {
        Ok(path) => {
            //apply the random wallpaper to windows wallpaper
            set_wallpaper(&path);
        }
        Err(err) => {
            println!("ERR: {}", err);
        }
    }
    }
}
