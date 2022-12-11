use std::fs::File;
use std::io::{self, BufRead};

struct Rng (u64,u64);

impl Rng {
    pub fn from(input: &str) -> Self {
        let (a, b) = input.split_once('-').unwrap();
        Self ( a.parse().unwrap(), b.parse().unwrap() )
    }

    fn contains(&self, other: &Rng) -> bool { self.0 <= other.0 && self.1 >= other.1 }

    fn overlaps(&self, other: &Rng) -> bool {
        (other.0 <= self.0 && self.0 <= other.1) || (other.0 <= self.1 && self.1 <= other.1)
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut contains_count: u64 = 0;
    let mut overlaps_count: u64 = 0;
    for line in lines {
        let line = line.unwrap();
        let (first, second) = line.split_once(',').unwrap();
        let first = Rng::from(first);
        let second = Rng::from(second);
        if first.contains(&second) || second.contains(&first) {
            contains_count += 1;
            overlaps_count += 1;
        } else if first.overlaps(&second) {
            overlaps_count += 1;
        }
    }
    println!("part 1 result {}", contains_count);
    println!("part 2 result {}", overlaps_count);

    Ok(())
}
