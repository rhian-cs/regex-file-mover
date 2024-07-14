use std::{error::Error, fs};

fn main() {
    if let Err(err) = read_files() {
        eprintln!("Failure: {err}");
    }
}

fn read_files() -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        let name = entry.file_name();

        println!("path = {:?}, name = {:?}", path, name);
    }

    Ok(())
}
