use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn main() {
    // Read file
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect_vec();

    // Part 1: Sum calories
    let mut elfs = lines
        .as_slice()
        .split(|line| line.is_empty())
        .map(|foods| {
            foods
                .iter()
                .map(|food| food.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect_vec();

    println!("{:?}", elfs.iter().max().unwrap());

    // Part 2: Sum top 3 calories
    elfs.sort_unstable();
    println!("{:?}", elfs.iter().rev().take(3).sum::<u32>());
}
