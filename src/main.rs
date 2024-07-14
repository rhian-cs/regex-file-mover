use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use regex::{Captures, Regex};

fn main() {
    if let Err(err) = read_files() {
        eprintln!("Failure: {err}");
    }
}

fn read_files() -> Result<(), Box<dyn Error>> {
    // let re = Regex::new(r"^[\d]{8}\.txt$").unwrap();
    // let re = Regex::new(r"^([\d]{4})([\d]{2})([\d]{2})\.txt$").unwrap();
    let re = Regex::new(r"^([\d]{4})([\d]{2})(01)\.txt$").unwrap();

    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        let name = entry.file_name();
        let name = name.to_str().ok_or("Could not parse filename")?;

        let mut new_path: PathBuf = match re.captures(name) {
            Some(captures) => build_new_dir(captures),
            None => Path::new("Uncategorized").into(),
        };

        println!("{:?} -> {:?}", path, new_path);

        fs::create_dir_all(&new_path)?;

        new_path.push(name);

        fs::rename(path, new_path)?;
    }

    Ok(())
}

fn build_new_dir(captures: Captures) -> PathBuf {
    captures
        .iter()
        .skip(1) // Skip whole regex match
        .map(|cap| cap.unwrap().as_str())
        .map(Path::new)
        .collect()
}
