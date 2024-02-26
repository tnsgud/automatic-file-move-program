use std::{fs, io, path::PathBuf};

fn get_files_in_directory(path: &str) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?;

    let files: Vec<PathBuf> = entries
        .filter_map(|entry| Some(entry.ok()?.path()))
        .collect();

    Ok(files)
}

fn main() {
    let mut folder_path = String::new();

    println!("Please input folder path");
    io::stdin()
        .read_line(&mut folder_path)
        .expect("Failed to read line");

    let folder_path = folder_path.trim();

    println!("{folder_path}");

    match get_files_in_directory(folder_path) {
        Ok(files) => {
            for file in files {
                if file.is_dir() {
                    if let Ok(m) = file.metadata() {
                        println!("{}", file.display());
                    }
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    };
}
