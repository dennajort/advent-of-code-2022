use std::fs::File;
use std::io::{self, BufRead};

fn item_type_value(item: char) -> u64 {
    match item {
        'a'..='z' => (item as u64) - ('a' as u64) + 1,
        'A'..='Z' => (item as u64) - ('A' as u64) + 27,
        _ => panic!("impossible value"),
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();
    
    let mut total_priorities: u64 = 0;
    let mut total_badges: u64 = 0;
    let mut line_stack = Vec::new();
    
    for line in lines {
        let line = line.unwrap();
        let (first, last) = line.split_at(line.len()/2);
        'outer: for fc in first.chars() {
            for lc in last.chars() {
                if fc == lc {
                    println!("{} {} {}", first, last, fc);
                    total_priorities += item_type_value(fc);
                    break 'outer;
                }
            }
        }
        line_stack.push(line);
        if line_stack.len() == 3 {
            'outer: for fc in line_stack[0].chars() {
                for sc in line_stack[1].chars() {
                    if fc != sc {
                        continue;
                    }
                    for tc in line_stack[2].chars() {
                        if fc == tc {
                            println!("common badge {}", fc);
                            total_badges += item_type_value(fc);
                            break 'outer;
                        }
                    }
                }
            }
            line_stack.clear();
        }
    }

    println!("part 1 result {}", total_priorities);
    println!("part 2 result {}", total_badges);

    Ok(())
}
