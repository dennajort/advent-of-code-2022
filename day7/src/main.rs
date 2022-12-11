use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

const SIZE_SMALL_DIR: u64 = 100000;
const TOTAL_DISK_SPACE: u64 = 70000000;
const SPACE_NEEDED: u64 = 30000000;

#[derive(Debug)]
enum FSElement {
    File(u64),
    Directory(HashMap<String, Self>),
}

impl FSElement {
    fn size(&self) -> u64 {
        match self {
            Self::File(size) => *size,
            Self::Directory(elements) => {
                elements.values().map(|e| -> u64 { e.size() }).sum()
            }
        }
    }

    fn add_element(&mut self, directories: &[String], name: &String, element: FSElement) {
        match self {
            Self::Directory(elements) => {
                if directories.len() == 0 {
                    elements.insert(name.clone(), element);
                } else {
                    let (directory, directories) = directories.split_first().unwrap();
                    let elem = elements.get_mut(directory).unwrap();
                    elem.add_element(directories, name, element);
                }
            }
            _ => panic!("only directories can add elements"),
        }
    }

    fn sum_size_big_dirs(&self) -> u64 {
        match self {
            Self::File(_) => 0,
            Self::Directory(elements) => {
                let mut total_size = 0;
                for e in elements.values() {
                    total_size += e.sum_size_big_dirs();
                }
                let size = self.size();
                if size <= SIZE_SMALL_DIR {
                    total_size += size;
                }
                total_size
            }
        }
    }

    fn find_best_dir(&self, space_needed: u64) -> Option<u64> {
        match self {
            Self::File(_) => None,
            Self::Directory(elements) => {
                let mut best_dir = None;
                for e in elements.values() {
                    let dir_size = e.find_best_dir(space_needed);
                    best_dir = match (best_dir, dir_size) {
                        (None, None) => None,
                        (None, Some(_)) => dir_size,
                        (Some(_), None) => best_dir,
                        (Some(b), Some(d)) => if b > d { dir_size } else { best_dir }
                    }
                }
                if best_dir.is_some() {
                    best_dir
                } else {
                    let size = self.size();
                    if size >= space_needed {
                        Some(size)
                    } else {
                        None
                    }
                }
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut fs = FSElement::Directory(HashMap::new());
    let mut dir_stack = Vec::new();

    for line in lines {
        let line = line.unwrap();
        println!("{}", line);
        if line.starts_with("$ cd ") {
            let path = &line[5..];
            match path {
                "/" => {
                    dir_stack = Vec::new();
                }
                ".." => {
                    dir_stack.pop();
                }
                name => {
                    dir_stack.push(name.to_string().clone());
                }
            }
        } else if line.starts_with("$ ls") {
        } else if line.starts_with("dir ") {
            let name = &line[4..];
            fs.add_element(dir_stack.as_slice(), &name.to_string(), FSElement::Directory(HashMap::new()));
        } else {
            let (size, name) = line.split_once(' ').unwrap();
            let size: u64 = size.parse().unwrap();
            fs.add_element(dir_stack.as_slice(), &name.to_string(), FSElement::File(size));
        }
    }

    println!("fs {:#?}", fs);

    let total_delete = fs.sum_size_big_dirs();

    let disk_used = fs.size();
    println!("disk used {}", disk_used);
    let disk_free = TOTAL_DISK_SPACE-disk_used;
    println!("disk free {}", disk_free);
    let to_delete = SPACE_NEEDED-disk_free;
    println!("need delete {}", to_delete);

    let deleted_dir_size = fs.find_best_dir(to_delete).unwrap();

    println!("part 1 result {}", total_delete);
    println!("part 2 result {}", deleted_dir_size);
    Ok(())
}
