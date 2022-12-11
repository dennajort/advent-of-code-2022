use std::fs::File;
use std::io::{self, BufRead};

fn all_different_elements(slice: &[u8]) -> bool {
    for i in 0..slice.len() {
        let slice = &slice[i..];
        let f = &slice[0];
        for c in &slice[1..] {
            if f == c {
                return false
            }
        }
    }
    return true
}

fn find_first_distinct_chars(input: &[u8], size: usize) -> Option<usize> {
    for n in size..input.len() {
        let slice = &input[n-size..n];
        if all_different_elements(slice) {
            return Some(n)
        }
    }
    return None
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut line = String::new();
    io::BufReader::new(file).read_line(&mut line)?;
    let line = line.as_bytes();

    println!("part 1 result {:?}", find_first_distinct_chars(line, 4));
    println!("part 2 result {:?}", find_first_distinct_chars(line, 14));

    Ok(())
}
