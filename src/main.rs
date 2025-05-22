use std::fs;
use std::{io::Write, path::PathBuf, thread, time};

use fastrand;
use reqwest;
use tokio;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use windows::Win32::UI::WindowsAndMessaging::{
    SPI_SETDESKWALLPAPER, SPIF_SENDWININICHANGE, SPIF_UPDATEINIFILE, SystemParametersInfoW,
};

// CFG
const SAVE_FETCHED_FILE: bool = false; // (true) when an image is fetched from the internet, it creats a copy of it instead rather than overwrite. 
const FETCHED_Y: u32 = 192; // sets the HEIGHT of the image when fetching from https://picsum.photos/1920/1080
const FETCHED_X: u32 = 180; // sets the WIDTH of the image when fetching from https://picsum.photos/1920/1080

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

// 1. gets data from api
// 2. creates directory and creates a file to put the picture in.
// 3. send the Result.
async fn fetch_wallpaper(i: u32) -> Result<PathBuf, String> {
    let mut file_path: String = "./tempImage/TranscodedWallpaper.jpg".to_string();

    if SAVE_FETCHED_FILE {
        file_path = format!("./tempImage/{}.jpg", i).to_string();
    }

    // get image from api
    let data = reqwest::get(format!("https://picsum.photos/{}/{}", FETCHED_X, FETCHED_Y))
        .await
        .map_err(|err| format!("No respone from API: {}", err))?
        .bytes()
        .await
        .map_err(|err| format!("Failed to make response into bytes: {}", err))?;

    fs::create_dir_all("./tempImage")
        .map_err(|err| format!("Failed creating ./tempImage dir: {}", err))?;

    let mut file = fs::File::create(&file_path)
        .map_err(|e| format!("Failed to create file in ./tempImage directory: {}", e))?; // creates or truncates the file.            

    file.write_all(&data).map_err(|err| {
        format!(
            "Failed to write data to created file 'fetch_wallpaper': {}",
            err
        )
    })?;

    let absolute_path = PathBuf::from(file_path).canonicalize().map_err(|err| {
        format!(
            "Error occured when retrieveing files absolute path: {}",
            err
        )
    })?;

    Ok(absolute_path)
}

// uses windows SystemParametersInfoW to set wallpaper
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

fn sleep(m: u64) {
    std::thread::sleep(std::time::Duration::from_millis(m));
}

#[tokio::main]
async fn main() {
    //let wallpapers_dir = PathBuf::from("C:/Users/kaspar/Desktop/Kaspar Files/Personal folder/Desktop images");

    for i in 0.. {
        sleep(500);
        //let wallpaper_path1 = get_random_wallpaper(&wallpapers_dir);
        let wallpaper_path2 = fetch_wallpaper(i).await;

        match wallpaper_path2 {
            Ok(path) => {
                //apply the random wallpaper to windows wallpaper

                println!("Setting wallpaper: {}", path.display());
                set_wallpaper(&path);
                println!("Wallpaper set");
            }
            Err(err) => {
                println!("ERR: {}", err);
            }
        }
    }
}
