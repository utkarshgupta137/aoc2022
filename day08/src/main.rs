use std::collections::HashSet;
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
    let trees = lines
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect_vec()
        })
        .collect_vec();
    let len = trees.len();

    // Part 1
    let mut visible = HashSet::new();
    for (i, row) in trees.iter().enumerate().take(len - 1).skip(1) {
        for (j, tree) in row.iter().enumerate().take(len - 1).skip(1) {
            if tree > row[..j].iter().max().unwrap()
                || tree > row[j + 1..].iter().max().unwrap()
                || *tree > trees[..i].iter().map(|row| row[j]).max().unwrap()
                || *tree > trees[i + 1..].iter().map(|row| row[j]).max().unwrap()
            {
                visible.insert((i, j, tree));
            }
        }
    }
    println!("{:?}", visible.len() + (len - 1) * 4);

    // Part 2
    let mut best = 0;
    for (i, row) in trees.iter().enumerate().take(len - 1).skip(1) {
        for (j, tree) in row.iter().enumerate().take(len - 1).skip(1) {
            let mut scores = (0, 0, 0, 0);
            for nb in row[..j].iter().rev() {
                scores.0 += 1;
                if nb >= tree {
                    break;
                }
            }
            for nb in row[j + 1..].iter() {
                scores.1 += 1;
                if nb >= tree {
                    break;
                }
            }
            for nb in trees[..i].iter().map(|row| row[j]).rev() {
                scores.2 += 1;
                if nb >= *tree {
                    break;
                }
            }
            for nb in trees[i + 1..].iter().map(|row| row[j]) {
                scores.3 += 1;
                if nb >= *tree {
                    break;
                }
            }
            let score = scores.0 * scores.1 * scores.2 * scores.3;
            if score > best {
                best = score;
            }
        }
    }
    println!("{:?}", best);
}
