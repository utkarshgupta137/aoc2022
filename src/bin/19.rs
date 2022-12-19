use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

const TIME1: i32 = 24;
const TIME2: i32 = 32;

#[derive(Debug)]
struct Blueprint {
    id: i32,
    robots: [[i32; 3]; 4],
    max_robots: [i32; 4],
}

fn get_next(iter: &mut regex::SubCaptureMatches) -> i32 {
    iter.next()
        .unwrap()
        .unwrap()
        .as_str()
        .parse::<i32>()
        .unwrap()
}

fn parse_file(lines: &[String]) -> Vec<Blueprint> {
    let re = Regex::new(concat!(
        r"Blueprint (\d+): ",
        r"Each ore robot costs (\d+) ore. ",
        r"Each clay robot costs (\d+) ore. ",
        r"Each obsidian robot costs (\d+) ore and (\d+) clay. ",
        r"Each geode robot costs (\d+) ore and (\d+) obsidian.",
    ))
    .unwrap();
    lines
        .iter()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            let mut iter = captures.iter();
            iter.next();
            let id = get_next(&mut iter);
            let robots = [
                [get_next(&mut iter), 0, 0],
                [get_next(&mut iter), 0, 0],
                [get_next(&mut iter), get_next(&mut iter), 0],
                [get_next(&mut iter), 0, get_next(&mut iter)],
            ];
            let max_robots = [
                robots.iter().map(|robot| robot[0]).max().unwrap(),
                robots.iter().map(|robot| robot[1]).max().unwrap(),
                robots.iter().map(|robot| robot[2]).max().unwrap(),
                TIME2,
            ];
            Blueprint {
                id,
                robots,
                max_robots,
            }
        })
        .collect_vec()
}

fn simulate(blueprint: Blueprint, time: i32) -> i32 {
    let robots = [1, 0, 0, 0];
    let materials = [0, 0, 0, 0];
    let make_all = [true, true, true, true];

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((time, make_all, robots, materials));

    let mut best = 0;
    while let Some((time, make, robots, mut materials)) = queue.pop_front() {
        best = best.max(materials[3]);
        if time == 0 || materials[3] + robots[3] * time + time * (time - 1) / 2 < best {
            continue;
        }

        let mut made = make_all;
        for (robot, requirements) in blueprint.robots.iter().enumerate() {
            if robots[robot] == blueprint.max_robots[robot] || !make[robot] {
                continue;
            }

            let mut flag = true;
            for (material, quantity) in requirements.iter().enumerate() {
                if materials[material] < *quantity {
                    flag = false;
                    break;
                }
            }

            if flag {
                made[robot] = false;
                let mut robots1 = robots;
                robots1[robot] += 1;
                let mut materials1 = materials;
                for (material, rate) in robots.iter().enumerate() {
                    materials1[material] += *rate;
                }
                for (material, quantity) in requirements.iter().enumerate() {
                    materials1[material] -= quantity;
                }

                let item = (time - 1, make_all, robots1, materials1);
                if !visited.contains(&item) {
                    visited.insert(item);
                    queue.push_back(item);
                }
            }
        }

        for (material, rate) in robots.iter().enumerate() {
            materials[material] += *rate;
        }
        let item = (time - 1, made, robots, materials);
        if !visited.contains(&item) {
            visited.insert(item);
            queue.push_back(item);
        }
    }
    best
}

pub fn part_one(lines: &[String]) -> Option<i32> {
    let blueprints = parse_file(lines);
    Some(
        blueprints
            .into_par_iter()
            .map(|blueprint| blueprint.id * simulate(blueprint, TIME1))
            .sum::<i32>(),
    )
}

pub fn part_two(lines: &[String]) -> Option<i32> {
    let blueprints = parse_file(lines);
    let mut prod = 1;
    for geodes in blueprints
        .into_par_iter()
        .take(3)
        .map(|blueprint| simulate(blueprint, TIME2))
        .collect::<Vec<_>>()
    {
        prod *= geodes;
    }
    Some(prod)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), Some(3472));
    }
}
