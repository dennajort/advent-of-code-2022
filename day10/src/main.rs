use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug)]
enum Cmd {
    Noop,
    Addx(i64),
}

impl Cmd {
    pub fn from(line: &str) -> Result<Cmd, &'static str>{
        if line == "noop" {
            return Ok(Self::Noop);
        }
        if line.starts_with("addx ") {
            if let Ok(val) = line[5..].parse() {
                return Ok(Self::Addx(val));
            }
        }
        return Err("invalid input");
    }

    fn cycles(&self) -> i64 {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut cmds: VecDeque<Cmd> = lines.map(|line| -> Cmd {
        let line = line.unwrap();
        Cmd::from(&line).unwrap()
    }).collect();

    println!("cmds {:?}", cmds);

    let mut reg_x = 1;
    let mut timer = 0;
    let mut curr_cmd = None;
    let mut strength_sum = 0;
    let mut screen = String::new();

    for cycle in 1..=240 {
        if timer == 0 {
            if let Some(Cmd::Addx(v)) = curr_cmd {
                reg_x += v;
            }
            curr_cmd = cmds.pop_front();
            if let Some(cmd) = curr_cmd {
                timer = cmd.cycles();
            }
        }
        // for part 1
        if (cycle - 20) % 40 == 0 {
            let strength = cycle * reg_x;
            println!("current cycle {cycle} strength {strength}");
            strength_sum += strength;
        }
        // for part 2
        let col = (cycle - 1) % 40;
        screen.push(if col >= reg_x-1 && col <= reg_x+1 { '#' } else { '.' });
        timer -= 1;
    }

    println!("part 1 result {}", strength_sum);
    println!("part 2");
    for n in 0..6 {
        println!("{}", &screen[n*40..(n+1)*40]);
    }
    Ok(())
}
