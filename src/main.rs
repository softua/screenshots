#[warn(clippy::all, clippy::pedantic)]
use chrono::Utc;
use path::PathBuf;
use rdev::{grab, Event, EventType, Key};
use screenshots::Screen;
use std::{env, fs, path};

const TARGET_DIR: &str = "screens";

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let screens_dir = args.get(1).unwrap_or(&TARGET_DIR.to_string()).to_string();
    let mut path: PathBuf = env::current_dir()?;
    path.push(screens_dir);
    fs::create_dir_all(&path)?;

    if let Err(error) = grab(move |e| callback(e, path.to_str().unwrap_or(TARGET_DIR).to_string()))
    {
        println!("Error: {error:?}");
    }

    Ok(())
}

fn callback(e: Event, path: String) -> Option<Event> {
    match e.event_type {
        EventType::KeyPress(Key::Alt) => {
            make_screen(path);
            None
        }
        _ => Some(e),
    }
}

fn make_screen(path: String) {
    let screens = Screen::all().unwrap();
    for screen in screens {
        let now = Utc::now();
        let image: screenshots::image::ImageBuffer<screenshots::image::Rgba<u8>, Vec<u8>> =
            screen.capture().unwrap();
        let filename = format!("{}/{}.jpg", path, now);
        let _ = image.save(filename);
    }
}
