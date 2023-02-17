use std::fs;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day_one()?;
    Ok(())
}

fn day_one() -> io::Result<()> {
    let str = fs::read_to_string("../inputs/day_one.txt")?;
    let v_calories = str.split("\r\n");
    let mut best_sum: u32 = 0;
    let mut best_elf: usize = 0;
    let mut partial_sum: u32 = 0;
    let mut most_calories: Vec<u32> = vec![];

    for (i, calories) in v_calories.enumerate() {
        match calories.parse::<u32>() {
            Ok(i_calories) => {
                partial_sum += i_calories;
            }
            Err(_error) => {
                if partial_sum > best_sum {
                    best_elf = i;
                    best_sum = partial_sum;
                }
                most_calories.push(partial_sum);
                partial_sum = 0;
            }
        }
    }
    most_calories.sort();

    println!("Best elf {}, with calorie count {}", best_elf, best_sum);
    println!("Calorie total for first three elves {:?}", &most_calories[most_calories.len() - 3..].iter().sum::<u32>());
    Ok(())
}
