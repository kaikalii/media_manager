use std::{env, fs, io, path::PathBuf, process::exit};

use clap::Parser;

#[derive(Parser)]
struct App {
    path: PathBuf,
    tags: Vec<String>,
}

fn main() {
    let app = App::parse();

    let dir = env::var("MEDIA_PATH").unwrap_or_else(|_| {
        eprintln!("You must set MEDIA_PATH");
        exit(1)
    });
    let dir = PathBuf::from(dir);

    let movie_path = app.path;
    let copy_to_tag_folder = |tag: &str| -> io::Result<()> {
        let tag_dir = dir.join(tag);
        fs::create_dir_all(&tag_dir)?;
        fs::hard_link(&movie_path, tag_dir.join(&movie_path))?;
        Ok(())
    };

    for tag in app.tags {
        if let Err(e) = copy_to_tag_folder(&tag) {
            eprintln!("failed to copy to {} tag folder: {}", tag, e);
        }
    }
}
