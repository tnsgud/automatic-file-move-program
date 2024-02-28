use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

fn get_files_in_directory(path: String) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?;

    let files: Vec<PathBuf> = entries
        .filter_map(|entry| Some(entry.ok()?.path()))
        .collect();

    Ok(files)
}

fn get_metadata(path: String) {
    match get_files_in_directory(path) {
        Ok(files) => {
            for file in files {
                if file.is_file() {
                    if let Ok(m) = file.metadata() {
                        println!("{file:?}//{m:?}");
                    }
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    };
}

fn main() {
    let file = File::open("path.txt").expect("hello");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => get_metadata(line),
            Err(err) => println!("{err}"),
        };
    }
}
