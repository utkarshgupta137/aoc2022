use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Move {
    fn from(input: &str) -> Self {
        match input {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let res = match *self {
            Self::Rock => match *other {
                Self::Rock => Ordering::Equal,
                Self::Paper => Ordering::Less,
                Self::Scissors => Ordering::Greater,
            },
            Self::Paper => match *other {
                Self::Rock => Ordering::Greater,
                Self::Paper => Ordering::Equal,
                Self::Scissors => Ordering::Less,
            },
            Self::Scissors => match *other {
                Self::Rock => Ordering::Less,
                Self::Paper => Ordering::Greater,
                Self::Scissors => Ordering::Equal,
            },
        };
        Some(res)
    }
}

fn get_score(p1: Move, p2: Move) -> i32 {
    let mut score = match p2 {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };

    if p1 < p2 {
        score += 6;
    } else if p1 == p2 {
        score += 3;
    }

    score
}

fn main() {
    // Read file
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect_vec();

    // Part 1
    let games = lines
        .iter()
        .map(|line| {
            let (i1, i2) = line.split_once(' ').unwrap();
            let p1 = Move::from(i1);
            let p2 = Move::from(i2);
            (p1, p2)
        })
        .collect_vec();

    println!(
        "{:?}",
        games
            .into_iter()
            .map(|(p1, p2)| get_score(p1, p2))
            .sum::<i32>()
    );

    // Part 2
    let games = lines
        .into_iter()
        .map(|line| {
            let (i1, i2) = line.split_once(' ').unwrap();
            let p1 = Move::from(i1);
            let res = match i2 {
                "X" => Ordering::Greater,
                "Y" => Ordering::Equal,
                "Z" => Ordering::Less,
                _ => unreachable!(),
            };

            let p2 = if p1.partial_cmp(&Move::Rock).unwrap() == res {
                Move::Rock
            } else if p1.partial_cmp(&Move::Paper).unwrap() == res {
                Move::Paper
            } else {
                Move::Scissors
            };

            (p1, p2)
        })
        .collect_vec();

    println!(
        "{:?}",
        games
            .into_iter()
            .map(|(p1, p2)| get_score(p1, p2))
            .sum::<i32>()
    );
}
