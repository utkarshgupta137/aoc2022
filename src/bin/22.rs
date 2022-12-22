use std::convert::TryInto;

use itertools::Itertools;

type Pos = (isize, isize, usize);
type Map = [[char; SHAPE.1]; SHAPE.0];

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const CHARS: [char; 4] = ['>', 'v', '<', '^'];

const SHAPE: (usize, usize) = (200, 150);
const START: Pos = (1, 51, 1);
const SIZE: isize = 50;

fn parse_file(lines: &[String]) -> Option<(Map, Vec<(usize, isize)>)> {
    let mut lines = lines.split(|line| line.is_empty());
    let map = lines
        .next()?
        .iter()
        .map(|line| line.chars().collect_vec().try_into().unwrap())
        .collect_vec()
        .try_into()
        .unwrap();
    let dirs = lines
        .next()?
        .first()?
        .split_inclusive(|chr: char| chr.is_alphabetic())
        .filter_map(|val| {
            if val.chars().last()?.is_alphabetic() {
                let (dist, chr) = val.split_at(val.len() - 1);
                let dir = if chr.chars().next()? == 'R' { 1 } else { 3 };
                Some((dist.parse::<usize>().ok()?, dir))
            } else {
                Some((val.parse::<usize>().ok()?, 0))
            }
        })
        .collect_vec();
    Some((map, dirs))
}

fn simulate(
    mut map: Map,
    dirs: Vec<(usize, isize)>,
    (mut y, mut x, mut dir): Pos,
    p2: bool,
) -> Pos {
    for (dist, rotate) in dirs {
        map[(y - 1) as usize][(x - 1) as usize] = CHARS[(dir - 1) as usize];
        'outer: for _ in 0..dist {
            let prev = (y, x, dir);
            let (dy, dx) = DIRS[(dir - 1) as usize];
            x += dx;
            y += dy;
            loop {
                if x < 1 || x > (SHAPE.1 as isize) || y < 1 || y > (SHAPE.0 as isize) {
                    if p2 {
                        (y, x, dir) = wrap(prev);
                        y += dy;
                        x += dx;
                        continue;
                    } else {
                        if x < 1 {
                            x = SHAPE.1 as isize;
                        }
                        if x > (SHAPE.1 as isize) {
                            x = 1;
                        }
                        if y < 1 {
                            y = SHAPE.0 as isize;
                        }
                        if y > (SHAPE.0 as isize) {
                            y = 1;
                        }
                    }
                }

                if map[(y - 1) as usize][(x - 1) as usize] == ' ' {
                    if p2 {
                        (y, x, dir) = wrap(prev);
                    } else {
                        y += dy;
                        x += dx;
                    }
                } else if map[(y - 1) as usize][(x - 1) as usize] == '#' {
                    (y, x, dir) = prev;
                    break 'outer;
                } else {
                    break;
                }
            }
            map[(y - 1) as usize][(x - 1) as usize] = CHARS[(dir - 1) as usize];
        }

        dir = ((dir as isize + rotate - 1) % 4) as usize + 1;
    }
    (y, x, dir)
}

fn wrap((y, x, dir): Pos) -> Pos {
    match ((y - 1) / SIZE, (x - 1) / SIZE, dir) {
        (0, _, 1) => (-y + 151, 100, 3),
        (1, _, 1) => (50, y + 50, 4),
        (2, _, 1) => (-y + 151, 150, 3),
        (3, _, 1) => (150, y - 100, 4),
        (0, _, 3) => (-y + 151, 1, 1),
        (1, _, 3) => (101, y - 50, 2),
        (2, _, 3) => (-y + 151, 51, 1),
        (3, _, 3) => (1, y - 100, 2),
        (_, 0, 2) => (1, x + 100, 2),
        (_, 1, 2) => (x + 100, 50, 3),
        (_, 2, 2) => (x - 50, 100, 3),
        (_, 0, 4) => (x + 50, 51, 1),
        (_, 1, 4) => (x + 100, 1, 1),
        (_, 2, 4) => (200, x - 100, 4),
        _ => unreachable!(),
    }
}

pub fn part_one(lines: &[String]) -> Option<isize> {
    let (map, dirs) = parse_file(lines)?;
    let (y, x, dir) = simulate(map, dirs, START, false);
    Some(1000 * y + 4 * x + (dir - 1) as isize)
}

pub fn part_two(lines: &[String]) -> Option<isize> {
    let (map, dirs) = parse_file(lines)?;
    let (y, x, dir) = simulate(map, dirs, START, true);
    Some(1000 * y + 4 * x + (dir - 1) as isize)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("inputs", 22);
        assert_eq!(part_one(&input), Some(77318));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("inputs", 22);
        assert_eq!(part_two(&input), Some(126017));
    }
}
