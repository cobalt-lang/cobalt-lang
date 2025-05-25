// HANDLING FILES AS U8 (helpful for bytecode files)

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::process;

pub fn read_file_to_vec(file_path: &str) -> Vec<u8> {
    let mut file = File::open(file_path).unwrap_or_else(|_| {
        eprintln!("Error: Could not open file '{}'", file_path);
        process::exit(1);
    });

    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap_or_else(|_| {
        eprintln!("Error: Could not read file '{}'", file_path);
        process::exit(1);
    });

    contents
}

pub fn write_vec_to_file(file_path: &str, data: &[u8]) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .unwrap_or_else(|_| {
            eprintln!("Error: Could not open file '{}'", file_path);
            process::exit(1);
        });

    file.write_all(data).unwrap_or_else(|_| {
        eprintln!("Error: Could not write to file '{}'", file_path);
        process::exit(1);
    });
}

pub fn get_file_name_without_extension(path: &str) -> Option<String> {
    let path = Path::new(path);
    
    path.file_stem().map(|filename| filename.to_string_lossy().to_string())
}