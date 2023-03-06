use std::{fs::{File}, path::Path, io::{BufReader, BufRead}};

pub fn open(path: &str) -> Vec<String>{
    let file = File::open(Path::new(path)).expect("Failed to open file");
    let reader = BufReader::new(file);

    reader.lines().filter_map(|line|line.ok()).collect()
}