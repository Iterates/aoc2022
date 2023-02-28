use std::{fs::{File}, io::{BufReader, BufRead}};

pub fn open(path: String) -> Vec<String>{
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);

    reader.lines().filter_map(|line|line.ok()).collect()
}