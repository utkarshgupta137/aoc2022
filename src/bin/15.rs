use std::collections::HashSet;

use itertools::Itertools;
use rayon::prelude::*;

type Dims = (isize, isize);

#[cfg(test)]
const Y: isize = 10;
#[cfg(test)]
const MAX: isize = 20;

#[cfg(not(test))]
const Y: isize = 2_000_000;
#[cfg(not(test))]
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

fn parse_file(lines: &[String]) -> Vec<(Dims, Dims, isize)> {
    lines
        .iter()
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
        .collect_vec()
}

pub fn part_one(lines: &[String]) -> Option<usize> {
    let s_b = parse_file(lines);
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
    Some(positions.len())
}

pub fn part_two(lines: &[String]) -> Option<isize> {
    let s_b = parse_file(lines);
    for (sensor, _, dist) in s_b.iter() {
        for pos in get_exterior(sensor, dist + 1) {
            let (x, y) = pos;
            if !(0..=MAX).contains(&x) || !(0..=MAX).contains(&y) {
                continue;
            }
            let mut flag = true;
            for (s, _, d) in s_b.iter() {
                if distance(s, &pos) <= *d {
                    flag = false;
                    break;
                }
            }
            if flag {
                return Some(x * 4_000_000 + y);
            }
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
