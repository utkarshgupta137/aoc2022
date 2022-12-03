use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn get_priority(c: char) -> u32 {
    let r = c.to_digit(36).unwrap() - 9;
    if c.is_uppercase() {
        r + 26
    } else {
        r
    }
}

fn main() {
    // Read file
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect_vec();

    // Part 1
    let rucksacks = lines.iter().map(|line| {
        let (c1, c2) = line.split_at(line.len() / 2);
        let h1: HashSet<char> = HashSet::from_iter(c1.chars());
        let h2: HashSet<char> = HashSet::from_iter(c2.chars());
        let c = h1.intersection(&h2).cloned().collect_vec()[0];
        get_priority(c)
    });
    println!("{:?}", rucksacks.sum::<u32>());

    // Part 2
    let badges = lines.as_slice().chunks(3).map(|group| {
        let mut group = group.iter().map(|rucksack| {
            let h: HashSet<char> = HashSet::from_iter(rucksack.chars());
            h
        });
        let h = group.next().unwrap();
        let c = group.fold(h, |acc, h| acc.intersection(&h).cloned().collect());
        get_priority(c.into_iter().next().unwrap())
    });
    println!("{:?}", badges.sum::<u32>());
}
