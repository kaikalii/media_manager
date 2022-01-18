use std::{fs, path::PathBuf, process::exit};

use clap::Parser;

#[derive(Parser)]
struct App {
    path: PathBuf,
    tags: Vec<String>,
}

fn main() {
    let app = App::parse();

    println!("will set tags of {:?} to:", app.path);

    let dir = std::env::var("MEDIA_PATH").unwrap_or_else(|_| {
        eprintln!("You must set MEDIA_PATH");
        exit(1)
    });
    let dir = PathBuf::from(dir);

    for tag in app.tags {
        let tag_dir = dir.join(tag);
        let _ = fs::create_dir_all(&tag_dir);
        let _ = fs::hard_link(&app.path, tag_dir.join(&app.path));
    }
}
