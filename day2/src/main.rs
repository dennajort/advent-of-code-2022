use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut first_score: u64 = 0;
    let mut second_score: u64 = 0;
    for line in lines {
        let line = line.unwrap();
        // A B C opponents rock paper scissors
        // X Y Z player rock paper scissors
        let score = match &line as &str {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            _ => panic!("impossible case"),
        };
        first_score += score;
        // A B C opponents rock paper scissors
        // X Y Z player lose draw win
        let score = match &line as &str {
            "A X" => 3 + 0,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2 + 0,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            _ => panic!("impossible case"),
        };
        second_score += score;
    }

    println!("part 1 result {}", first_score);
    println!("part 2 result {}", second_score);

    Ok(())
}
