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

    // Parse file
    let mut split = lines.split(|line| line.is_empty());

    let rows = split.next().unwrap();
    let len = rows
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut cases = vec![Vec::new(); len];
    for row in rows[0..rows.len() - 1].iter() {
        for (i, case) in row
            .chars()
            .collect_vec()
            .chunks(4)
            .map(|row| row[1])
            .enumerate()
            .filter(|(_i, case)| *case != ' ')
        {
            cases[i].insert(0, case);
        }
    }

    let ops = split
        .next()
        .unwrap()
        .iter()
        .map(|line| {
            let mut split = line.split_whitespace();
            (
                split.nth(1).unwrap().parse::<usize>().unwrap(),
                split.nth(1).unwrap().parse::<usize>().unwrap() - 1,
                split.nth(1).unwrap().parse::<usize>().unwrap() - 1,
            )
        })
        .collect_vec();

    // Part 1
    let mut cases1 = cases.clone();
    for (n, i, j) in ops.iter() {
        let len = cases1[*i].len() - n;
        let mut vals = cases1[*i].drain(len..).rev().collect();
        cases1[*j].append(&mut vals);
    }
    println!(
        "{}",
        cases1
            .iter()
            .map(|row| row.last().unwrap())
            .collect::<String>()
    );

    // Part 2
    let mut cases2 = cases.clone();
    for (n, i, j) in ops.iter() {
        let len = cases2[*i].len() - n;
        let mut vals = cases2[*i].drain(len..).collect();
        cases2[*j].append(&mut vals);
    }
    println!(
        "{}",
        cases2
            .iter()
            .map(|row| row.last().unwrap())
            .collect::<String>()
    );
}
