use std::collections::{HashSet, VecDeque};

use cached::proc_macro::cached;
use glam::i32::IVec2;
use itertools::Itertools;
use sark_grids::directions::*;
use sark_grids::grid::Grid;

type Pos = IVec2;
type Map = Grid<[bool; 4]>;

#[cfg(test)]
const MAP_LCM: usize = 12;
#[cfg(not(test))]
const MAP_LCM: usize = 300;

fn parse_file(lines: &[String]) -> Map {
    let map = lines
        .iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut grid = Grid::new([false; 4], [map.first().unwrap().len() - 2, map.len() - 2]);
    for (y, row) in map.into_iter().skip(1).take(grid.height()).enumerate() {
        for (x, chr) in row.into_iter().skip(1).take(grid.width()).enumerate() {
            let obj = match chr {
                '.' => [false, false, false, false],
                '^' => [true, false, false, false],
                '>' => [false, true, false, false],
                'v' => [false, false, true, false],
                '<' => [false, false, false, true],
                _ => unreachable!(),
            };
            grid.insert_row_at([x, y], [obj].into_iter());
        }
    }
    grid
}

#[cached(key = "String", convert = r##"{ format!("{}", _minutes % MAP_LCM) }"##)]
fn get_map(map: Option<&Map>, _minutes: usize) -> Map {
    let map = map.unwrap();
    let mut grid = Grid::new([false; 4], [map.width(), map.height()]);
    for (y, row) in map.iter_rows(0..grid.height()).enumerate() {
        for (x, dirs) in row.iter().enumerate() {
            let (up, right, down, left) = (dirs[0], dirs[1], dirs[2], dirs[3]);
            if up {
                let (ny, nx) = if y == 0 {
                    (grid.height() - 1, x)
                } else {
                    (y - 1, x)
                };
                grid[[nx, ny]][0] = true;
            }
            if right {
                let (ny, nx) = if x == grid.width() - 1 {
                    (y, 0)
                } else {
                    (y, x + 1)
                };
                grid[[nx, ny]][1] = true;
            }
            if down {
                let (ny, nx) = if y == grid.height() - 1 {
                    (0, x)
                } else {
                    (y + 1, x)
                };
                grid[[nx, ny]][2] = true;
            }
            if left {
                let (ny, nx) = if x == 0 {
                    (y, grid.width() - 1)
                } else {
                    (y, x - 1)
                };
                grid[[nx, ny]][3] = true;
            }
        }
    }
    grid
}

#[allow(clippy::collapsible_if)]
fn simulate((entry, start): (Pos, usize), exit: Pos) -> Option<usize> {
    let mut min_minutes = usize::MAX;
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((entry, start));

    while let Some((pos, minutes)) = queue.pop_front() {
        if minutes > min_minutes {
            continue;
        }
        if pos == exit {
            min_minutes = min_minutes.min(minutes);
            continue;
        }
        #[cfg(test)]
        {
            let map = get_map(None, minutes);
            println!("Minute: {}", minutes);
            print(&map, &pos);
        }
        let map = get_map(None, minutes + 1);
        for dir in [UP, RIGHT, DOWN, LEFT, Pos::ZERO] {
            let npos = pos + dir;
            if (map.in_bounds(npos) && map[npos] == [false; 4]) || npos == entry || npos == exit {
                if !visited.contains(&(npos, (minutes + 1) % MAP_LCM)) {
                    visited.insert((npos, (minutes + 1) % MAP_LCM));
                    queue.push_back((npos, minutes + 1));
                }
            }
        }
    }
    Some(min_minutes)
}

#[cfg(test)]
fn print(map: &Map, pos: &Pos) {
    for (y, row) in map.iter_rows(0..map.height()).enumerate() {
        println!(
            "{}",
            row.iter()
                .enumerate()
                .map(|(x, val)| {
                    if Pos::new(x as i32, y as i32) == *pos {
                        assert!(*val == [false; 4]);
                        "E".to_string()
                    } else {
                        match val {
                            [false, false, false, false] => ".".to_string(),
                            [true, false, false, false] => "^".to_string(),
                            [false, true, false, false] => ">".to_string(),
                            [false, false, true, false] => "v".to_string(),
                            [false, false, false, true] => "<".to_string(),
                            other => other.iter().map(|val| *val as u8).sum::<u8>().to_string(),
                        }
                    }
                })
                .collect::<String>()
        );
    }
    println!();
}

pub fn part_one(lines: &[String]) -> Option<usize> {
    #[cfg(test)]
    println!();
    let map = parse_file(lines);

    let mut grid = map.clone();
    for minutes in 1..=MAP_LCM {
        grid = get_map(Some(&grid), minutes);
    }

    let entry = Pos::new(0, -1);
    let exit = Pos::new((map.width() - 1) as i32, map.height() as i32);
    simulate((entry, 0), exit)
}

pub fn part_two(lines: &[String]) -> Option<usize> {
    #[cfg(test)]
    println!();
    let map = parse_file(lines);

    let mut grid = map.clone();
    for minutes in 1..=MAP_LCM {
        grid = get_map(Some(&grid), minutes);
    }

    let entry = Pos::new(0, -1);
    let exit = Pos::new((map.width() - 1) as i32, map.height() as i32);
    let p1 = simulate((entry, 0), exit)?;
    let p2 = simulate((exit, p1), entry)?;
    simulate((entry, p2), exit)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), Some(54));
    }
}
