mod args;

use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process,
    time::Instant,
};

use args::Args;
use regex::{Captures, Regex};

fn main() {
    if let Err(err) = read_and_process_files() {
        eprintln!("Failure: {err}");
        process::exit(1);
    }
}

fn read_and_process_files() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();

    let args = Args::parse()?;
    let re = Regex::new(args.pattern.as_str()).unwrap();
    let mut total_count = 0;
    let mut uncategorized_count = 0;

    for entry in fs::read_dir(args.directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        let name = entry.file_name();
        let name = name.to_str().ok_or("Could not parse filename")?;

        total_count += 1;

        let new_directory: PathBuf = match re.captures(name) {
            Some(captures) => build_new_dir(captures),
            None => {
                uncategorized_count += 1;
                Path::new(&args.uncategorized_directory).into()
            }
        };
        let mut new_path = new_directory.clone();
        new_path.push(name);

        let run_mode = if args.wet_run { "wet-run" } else { "dry-run" };
        println!(
            "[{run_mode}] \"{}\" {} -> {}",
            name,
            path.to_str().unwrap(),
            new_path.to_str().unwrap()
        );

        if args.wet_run {
            fs::create_dir_all(&new_directory)?;
            fs::rename(path, new_path)?;
        }
    }

    if args.wet_run {
        print!("\n{total_count} files were processed.");
    } else {
        print!("\n{total_count} files were found.");
    }
    if uncategorized_count > 0 {
        print!(" {uncategorized_count} files are uncategorized due to not matching the pattern.");
    } else {
        print!(" No uncategorized files.");
    }
    println!("");

    if args.wet_run {
        println!("Took {} seconds.", start_time.elapsed().as_secs_f32());
    } else {
        println!("No files were moved. Use --wet-run to apply the changes.");
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
