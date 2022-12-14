use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;
use rayon::prelude::*;

type Dims = (isize, isize);

// const FILE: &str = "sample.txt";
// const Y: isize = 10;
// const MAX: isize = 20;

const FILE: &str = "input.txt";
const Y: isize = 2_000_000;
const MAX: isize = 4_000_000;

fn distance((sx, sy): &Dims, (bx, by): &Dims) -> isize {
    (sx.abs_diff(*bx) + sy.abs_diff(*by)) as isize
}

fn get_exterior((sx, sy): &Dims, dist: isize) -> Vec<Dims> {
    (0..dist)
        .into_par_iter()
        .flat_map(|d| {
            [
                (sx + dist - d, sy - d),
                (sx - d, sy - dist + d),
                (sx - dist + d, sy + d),
                (sx + d, sy + dist - d),
            ]
            .into_par_iter()
        })
        .collect()
}

fn main() {
    // Read file
    let s_b = BufReader::new(File::open(FILE).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (sensor, beacon) = line.split_once(':').unwrap();
            let sensor = sensor.split_once("x=").unwrap().1;
            let beacon = beacon.split_once("x=").unwrap().1;
            let (sx, sy) = sensor.split_once(", y=").unwrap();
            let (bx, by) = beacon.split_once(", y=").unwrap();
            let sensor: Dims = (sx.parse::<isize>().unwrap(), sy.parse::<isize>().unwrap());
            let beacon: Dims = (bx.parse::<isize>().unwrap(), by.parse::<isize>().unwrap());
            (sensor, beacon, distance(&sensor, &beacon))
        })
        .collect_vec();

    // Part 1
    let positions = s_b
        .par_iter()
        .flat_map(|(sensor, beacon, dist)| {
            ((sensor.0 - dist)..=(sensor.0 + dist))
                .into_par_iter()
                .map(|x| (x, Y))
                .filter(move |pos| pos != sensor && pos != beacon)
                .filter(move |pos| distance(sensor, pos) <= *dist)
        })
        .collect::<HashSet<_>>();
    println!("{:?}", positions.len());

    // Part 2
    'outer: for (sensor, _, dist) in s_b.iter() {
        for pos in get_exterior(sensor, dist + 1) {
            let (x, y) = pos;
            if !(0..=MAX).contains(&x) || !(0..=MAX).contains(&y) {
                continue;
            }
            for (s, _, d) in s_b.iter() {
                if distance(s, &pos) <= *d {
                    println!("{:?}: {}", (x, y), x * 4_000_000 + y);
                    break 'outer;
                }
            }
        }
    }
}
