#![allow(non_snake_case)] // Disable goofy error

// Libraries
use std::fs::{OpenOptions, read_to_string, read_dir};
use std::io::{self, Write};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


fn main() {
    println!("Hello, world!");
    if let Err(e) = write_task_to_file("Hello world!\n") {
        eprintln!("Failed to write to file: {}", e);
    }

    let content = read_task_from_file();
    println!("{}", content);
    let size = content.len();
    println!("{}", size);
}

fn write_task_to_file(task: &str) -> io::Result<()> {
    let file_name: String = generate_file_name();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)?;
    
    file.write_all(task.as_bytes())?; 
    println!("Successfully saved!");
    Ok(())
}

fn read_task_from_file() -> String {
    return read_to_string("tasks.txt").unwrap();
} 

fn generate_file_name() -> String {
    let rng = thread_rng();

    let random_string: String = rng
        .sample_iter(Alphanumeric)
        .filter(|&c| c.is_ascii_lowercase())
        .take(7)
        .map(|c| c as char)  // Convert each u8 to char
        .collect();

    return random_string;
}

fn get_files() -> Result<Vec<String>, std::io::Error> {
    let entries = read_dir(".")?;

    let files: Vec<_> = entries
        .filter_map(|entry| {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    Some(path.file_name()?.to_string_lossy().into_owned())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    Ok(files)
}