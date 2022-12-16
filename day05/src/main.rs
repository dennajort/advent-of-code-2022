use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut lines = io::BufReader::new(file).lines();

    let mut map = Vec::<VecDeque<char>>::with_capacity(9);
    map.resize(9, VecDeque::new());

    let mut map_sec = Vec::<VecDeque<char>>::with_capacity(9);
    map_sec.resize(9, VecDeque::new());

    loop {
        let line = lines.next().unwrap().unwrap();
        if line.starts_with(" 1") {
            lines.next();
            break;
        }
        for (step, idx) in (1..34).step_by(4).enumerate() {
            let c = line.chars().nth(idx).unwrap();
            if c != ' ' {
                map[step].push_back(c);
                map_sec[step].push_back(c);
            }
        }
    }

    println!("map {:?}", map);

    for line in lines {
        let line = line.unwrap();
        println!("{}", line);
        let chunks: Vec<&str> =  line.split_whitespace().collect();
        let qty = chunks[1].parse().unwrap();
        let src = chunks[3].parse::<usize>().unwrap() - 1;
        let dst = chunks[5].parse::<usize>().unwrap() - 1;

        let mut buf = Vec::with_capacity(qty);

        for _ in 0..qty {
            let crte = map[src].pop_front().unwrap();
            map[dst].push_front(crte);

            buf.push(map_sec[src].pop_front().unwrap());
        }

        for crte in buf.iter().rev() {
            map_sec[dst].push_front(*crte);
        }
    }

    let result: String = map.iter().map(|s| s.front().unwrap_or(&' ')).collect();
    let result_sec: String = map_sec.iter().map(|s| s.front().unwrap_or(&' ')).collect();

    println!("part 1 result '{}'", result);
    println!("part 2 result '{}'", result_sec);

    Ok(())
}
