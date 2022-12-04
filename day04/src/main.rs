use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn parse(line: &str) -> (u32, u32) {
    let (i, j) = line.split_once('-').unwrap();
    (i.parse::<u32>().unwrap(), j.parse::<u32>().unwrap())
}

fn main() {
    // Read file
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect_vec();

    // Part 1
    let sections = lines.iter().map(|line| {
        let (s1, s2) = line.split_once(',').unwrap();
        let (i1, j1) = parse(s1);
        let (i2, j2) = parse(s2);

        (match i1.cmp(&i2) {
            Ordering::Greater => j1 <= j2,
            Ordering::Less => j1 >= j2,
            Ordering::Equal => true,
        }) as u32
    });
    println!("{:?}", sections.sum::<u32>());

    // Part 2
    let sections = lines.iter().map(|line| {
        let (s1, s2) = line.split_once(',').unwrap();
        let (i1, j1) = parse(s1);
        let (i2, j2) = parse(s2);

        (if i1 > j2 { false } else { i2 <= j1 }) as u32
    });
    println!("{:?}", sections.sum::<u32>());
}
