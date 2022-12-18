use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

type Cube = (i32, i32, i32);

fn parse_file(lines: &[String]) -> Vec<Cube> {
    lines
        .iter()
        .map(|line| {
            let mut iter = line.splitn(3, ',');
            let a = iter.next().unwrap().parse::<i32>().unwrap();
            let b = iter.next().unwrap().parse::<i32>().unwrap();
            let c = iter.next().unwrap().parse::<i32>().unwrap();
            (a, b, c)
        })
        .sorted_unstable()
        .collect_vec()
}

fn naive_surface_area(cubes: &[Cube]) -> i32 {
    let mut exposed = (cubes.len() * 6) as i32;
    for (x1, y1, z1) in cubes.iter() {
        for (x2, y2, z2) in cubes.iter() {
            let dx = x1.abs_diff(*x2);
            let dy = y1.abs_diff(*y2);
            let dz = z1.abs_diff(*z2);
            if matches!((dx, dy, dz), (1, 0, 0) | (0, 1, 0) | (0, 0, 1)) {
                exposed -= 1;
            }
        }
    }
    exposed
}

fn is_free(cubes: &HashSet<Cube>, cube: Cube) -> Option<HashSet<Cube>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(cube);

    while let Some((x, y, z)) = queue.pop_front() {
        if x == 0 || y == 0 || z == 0 {
            return Some(visited);
        }

        for (dx, dy, dz) in [
            (0, 0, 1),
            (0, 0, -1),
            (0, 1, 0),
            (0, -1, 0),
            (1, 0, 0),
            (-1, 0, 0),
        ] {
            let cube = (x + dx, y + dy, z + dz);
            if !visited.contains(&cube) && !cubes.contains(&cube) {
                visited.insert(cube);
                queue.push_back(cube);
            }
        }
    }
    None
}

pub fn part_one(lines: &[String]) -> Option<i32> {
    let cubes = parse_file(lines);

    Some(naive_surface_area(&cubes))
}

pub fn part_two(lines: &[String]) -> Option<i32> {
    let cubes = parse_file(lines);
    let cubes_set = cubes.iter().cloned().collect::<HashSet<_>>();

    let mut trapped = Vec::new();
    let mut visited = HashSet::<Cube>::new();
    for x in 0..20 {
        for y in 0..20 {
            for z in 0..20 {
                let cube = (x, y, z);
                if cubes_set.contains(&cube) || visited.contains(&cube) {
                    continue;
                }
                if let Some(visits) = is_free(&cubes_set, cube) {
                    visited.extend(visits.iter());
                } else {
                    trapped.push(cube);
                }
            }
        }
    }

    Some(naive_surface_area(&cubes) - naive_surface_area(&trapped))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
