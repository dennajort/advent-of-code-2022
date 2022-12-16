use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn new(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("impossible"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos(i64, i64);

impl Pos {
    fn go(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.1 += 1,
            Direction::Down => self.1 -= 1,
            Direction::Right => self.0 += 1,
            Direction::Left => self.0 -= 1,
        }
    }

    fn follow(&mut self, head: &Pos) {
        let x_jump = self.0.abs_diff(head.0) > 1;
        let y_jump = self.1.abs_diff(head.1) > 1;

        if x_jump {
            if head.0 > self.0 {
                self.0 += 1;
            } else {
                self.0 -= 1;
            }
            if !y_jump {
                self.1 = head.1;
            }
        } 
        if y_jump {
            if head.1 > self.1 {
                self.1 += 1;
            } else {
                self.1 -= 1;
            }
            if !x_jump {
                self.0 = head.0;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut knots = [Pos(0, 0); 10];
    let mut second_history = HashMap::new();
    let mut tail_history = HashMap::new();

    for line in lines {
        let line = line.unwrap();
        if line == "" {
            continue;
        }
        let (dir, count) = line.split_once(' ').unwrap();
        let count = count.parse().unwrap();
        let dir = Direction::new(dir);
        for _ in 0..count {
            knots[0].go(&dir);
            for n in 1..10 {
                let prev = knots[n-1];
                knots[n].follow(&prev);
            }
            second_history.insert(knots[1].clone(), ());
            tail_history.insert(knots[9].clone(), ());
        }
    }

    println!("part 1 result {}", second_history.len());
    println!("part 2 result {}", tail_history.len());
    Ok(())
}
