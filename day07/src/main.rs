use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn insert(sizes: &mut HashMap<String, u32>, path: &Vec<String>, idx: usize, size: u32) {
    if idx < path.len() {
        sizes
            .entry(path[0..=idx].join(" "))
            .and_modify(|v| *v += size)
            .or_insert(size);
        insert(sizes, path, idx + 1, size);
    }
}

fn main() {
    // Read file
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect_vec();

    // Parse file
    let mut sizes = HashMap::new();
    let mut path = Vec::new();
    for line in lines {
        if let Some((_, dir)) = line.split_once("$ cd ") {
            if dir == ".." {
                path.pop();
            } else {
                path.push(dir.to_string());
            }
        } else if line != "$ ls" && !line.starts_with("dir ") {
            let (size, _) = line.split_once(' ').unwrap();
            insert(&mut sizes, &path, 0, size.parse::<u32>().unwrap());
        }
    }
    println!("{:?}", sizes);

    // Part 1
    let mut total = 0;
    for size in sizes.values() {
        if *size <= 100_000 {
            total += size;
        }
    }
    println!("{:?}", total);

    // Part 2
    let required = sizes["/"] - 40_000_000;
    let mut curr = sizes["/"];
    for size in sizes.values() {
        if *size >= required && *size < curr {
            curr = *size;
        }
    }
    println!("{:?}", curr);
}
