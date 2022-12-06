use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();
    
    let mut calories = Vec::new();
    let mut current_calories: u64 = 0;
    for line in lines {
        if let Ok(line) = line {
            if line == "" {
                calories.push(current_calories);
                current_calories = 0;
                continue;
            } else {
                current_calories += line.parse::<u64>().unwrap();
            }
        }
    }
    calories.sort_by(|a,b| b.cmp(a));
    calories.truncate(3);
    println!("part 1 result {}", calories[0]);
    println!("part 2 result {}", calories.iter().sum::<u64>());
    Ok(())
}
