use std::{fs::{File}, io::{BufReader, BufRead}, error::Error};
use std::collections::HashMap;

fn process_file(filename: &str, map: &mut HashMap<String, i32>) -> std::io::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| map.get_mut(&line).map(|count| *count += 1))
        .last();
    println!("{:?}", map);
    total_score(map)?;
    Ok(())
}

fn total_score(map: &mut HashMap<String, i32>) -> std::io::Result<()>{
    let scores = HashMap::from([("A X".to_string(), 2), ("A Y".to_string(), 8), ("A Z".to_string(), 3), ("B X".to_string(), 1), ("B Y".to_string(), 4), ("B Z".to_string(), 9), ("C X".to_string(), 7), ("C Y".to_string(), 2), ("C Z".to_string(), 6)]);
    let (_, sum_values) = map.iter().fold((0, 0), |(acc_keys, acc_values), (key, value)|{
        (acc_keys, acc_values + value * scores.get(key).unwrap())
    });
    println!("Score {}", sum_values);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>>{
    let totals = &mut HashMap::from([("A X".to_string(), 0), ("A Y".to_string(), 0), ("A Z".to_string(), 0), ("B X".to_string(), 0), ("B Y".to_string(), 0), ("B Z".to_string(), 0), ("C X".to_string(), 0), ("C Y".to_string(), 0), ("C Z".to_string(), 0)]);
    process_file("../inputs/day_two.txt", totals)?;
    Ok(())
}
