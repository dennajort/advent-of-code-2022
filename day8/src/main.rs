use std::fs::File;
use std::io::{self, BufRead};

fn find_first_taller_tree(trees: &mut dyn Iterator<Item = &u8>, size: &u8) -> u64 {
    let mut result = 0;
    for (n, t) in trees.enumerate() {
        result = n+1;
        if t >= size {
            break;
        }
    }
    result.try_into().unwrap()
}

fn main() -> std::io::Result<()> {

    let map = {
        let file = File::open("input.txt")?;
        let lines = io::BufReader::new(file).lines();

        let mut map = Vec::new();

        for line in lines {
            let line = line.unwrap();
            if line == "" {
                continue;
            }
            let line: Vec<u8> = line.chars().map(|c| c.to_string().parse::<u8>().unwrap()).collect();
            map.push(line);
        }

        map
    };

    // println!("map {:?}", map);
    println!("map size {} {} {}", map.len(), &map[0].len(), map.len()*&map[0].len());

    let mut visible_count: u64 = 0;
    for (x, line) in map.iter().enumerate() {
        for (y, t) in line.iter().enumerate() {
            // check line begin
            if line[..y].iter().all(|tt| tt < t) {
                visible_count += 1;
                continue;
            }
            // check line end
            if line[y+1..].iter().all(|tt| tt < t) {
                visible_count += 1;
                continue;
            }
            // check column begin
            if map[..x].iter().all(|line| &line[y] < t) {
                visible_count += 1;
                continue;
            }
            // check column end
            if map[x+1..].iter().all(|line| &line[y] < t) {
                visible_count += 1;
                continue;
            }
        }
    }

    let mut best_score: u64 = 0;
    for (x, line) in map.iter().enumerate() {
        for (y, t) in line.iter().enumerate() {
            // check line begin
            let score = find_first_taller_tree(&mut line[..y].iter().rev(), t);
            // check line end
            let score = score * find_first_taller_tree(&mut line[y+1..].iter(), t);
            // check column begin
            let score = score * find_first_taller_tree(
                &mut map[..x].iter().map(|line| &line[y]).rev(), t);
            // check column end
            let score = score * find_first_taller_tree(
                &mut map[x+1..].iter().map(|line| &line[y]), t);
            if score > best_score {
                best_score = score;
            }
        }
    }

    println!("part 1 result {}", visible_count);
    println!("part 2 result {}", best_score);
    Ok(())
}
