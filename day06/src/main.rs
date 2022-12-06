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
    let chars = lines.first().unwrap().chars().collect_vec();

    // Part 1
    for (i, window) in chars.windows(4).enumerate() {
        let set: HashSet<&char> = HashSet::from_iter(window);
        if set.len() == 4 {
            println!("{:?}", i + 4);
            break;
        }
    }

    // Part 2
    for (i, window) in chars.windows(14).enumerate() {
        let set: HashSet<&char> = HashSet::from_iter(window);
        if set.len() == 14 {
            println!("{:?}", i + 14);
            break;
        }
    }
}
