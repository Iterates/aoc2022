use std::{fs::{File}, io::{BufReader, BufRead, Lines}, error::Error, iter::Enumerate, collections::HashSet};
use std::array;

fn open() -> Lines<BufReader<File>>{
    let file = File::open("../inputs/day_three.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    reader.lines()
}

fn find_common_item(){
    let mut total_value: u32 = 0;
    let lines = open();

    lines.filter_map(|l|l.ok()).for_each(|l| {
        for char in &l.as_bytes()[..l.len()/2]{
          let index_option = l.as_bytes()[l.len()/2..].into_iter().position(|&second_char|second_char == *char);
          if index_option.is_none() {}
          else {
            let index = index_option.unwrap();
            let char_code = l.as_bytes()[l.len()/2..].get(index).unwrap();
            if char_code >= &97 {total_value += *char_code as u32 - 96} else { total_value += *char_code as u32 - 38}
            break;
          }
        }
    });
    println!("{}", total_value);
}

fn find_common_item_v2(){
  let lines: Vec<String> = open().filter_map(|line|line.ok()).collect();
  let mut sum = 0;
  for i in (0..lines.len()).step_by(3){
    let x: HashSet<char> = lines[i].chars().collect(); 
    let y: HashSet<char> = lines[i+1].chars().collect(); 
    let z: HashSet<char> = lines[i+2].chars().collect(); 
    let common: HashSet<char> = x.intersection(&y).copied().collect::<HashSet<char>>().intersection(&z).copied().collect::<HashSet<char>>();
    common.iter().for_each(|char| {
        let char_code = *char as u32;
        sum +=  if char_code >= 97 {char_code - 96} else {char_code - 38};  
    })
  }
  println!("{}", sum);
}

fn main() -> Result<(), Box<dyn Error>> {
    //find_common_item();
    find_common_item_v2();

    Ok(())
}
