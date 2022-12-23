#![allow(unused)]

use std::collections::HashMap;
use std::convert::TryInto;

use glam::i32::IVec2;
use itertools::Itertools;
use sark_grids::directions::*;
use sark_grids::grid::Grid;
use sark_grids::pivot::{Pivot, PivotedPoint};
use sark_grids::point::{GridPoint, Size2d};

type Map = Grid<char>;

const DIRS: [([IVec2; 3], IVec2); 4] = [
    ([DOWN, DOWN_LEFT, DOWN_RIGHT], DOWN),
    ([UP, UP_LEFT, UP_RIGHT], UP),
    ([LEFT, UP_LEFT, DOWN_LEFT], LEFT),
    ([RIGHT, UP_RIGHT, DOWN_RIGHT], RIGHT),
];

fn parse_file(lines: &[String]) -> Option<Map> {
    let mat = lines
        .iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut grid = Grid::default([mat.first()?.len(), mat.len()]);
    for (y, row) in mat.into_iter().enumerate() {
        grid.insert_row(y, row.into_iter());
    }
    Some(extend(grid))
}

fn extend(map: Map) -> Map {
    let mut flag = is_empty(map.iter_row(0).copied());
    if flag {
        let mut grid = Grid::new('.', [map.width(), map.height() + 1]);
        return extend(copy_grid(map, grid, 0, 1));
    }

    let mut flag = is_empty(map.iter_row(map.height() - 1).copied());
    if flag {
        let mut grid = Grid::new('.', [map.width(), map.height() + 1]);
        return extend(copy_grid(map, grid, 0, 0));
    }

    let mut flag = is_empty(map.iter_column(0).copied());
    if flag {
        let mut grid = Grid::new('.', [map.width() + 1, map.height()]);
        return extend(copy_grid(map, grid, 1, 0));
    }

    let mut flag = is_empty(map.iter_column(map.width() - 1).copied());
    if flag {
        let mut grid = Grid::new('.', [map.width() + 1, map.height()]);
        return extend(copy_grid(map, grid, 0, 0));
    }
    map
}

fn is_empty(axis: impl DoubleEndedIterator<Item = char>) -> bool {
    let mut flag = false;
    for chr in axis {
        if chr == '#' {
            flag = true;
            break;
        }
    }
    flag
}

fn copy_grid(src: Map, mut dst: Map, x: usize, y: usize) -> Map {
    for (i, row) in src.iter_rows(0..src.height()).enumerate() {
        dst.insert_row_at([x, y + i], row.iter().copied());
    }
    dst
}

fn print(map: &Map) {
    println!();
    for row in map.iter_rows(0..map.height()) {
        println!("{}", row.iter().collect::<String>());
    }
}

fn simulate(mut map: Map, skip: usize) -> Option<Map> {
    let mut elves = HashMap::new();
    for (y, row) in map.iter_rows(0..map.height()).enumerate() {
        for (x, chr) in row.iter().enumerate() {
            if *chr == '#' {
                elves.insert(IVec2::new(x as i32, y as i32), None);
            }
        }
    }
    for (elf, pos) in elves.iter_mut() {
        let mut flag = false;
        for dir in DIR_8 {
            if map.in_bounds(*elf + *dir) && map[*elf + *dir] == '#' {
                flag = true;
            }
        }
        if !flag {
            continue;
        }
        for dirs in DIRS.iter().cycle().skip(skip).take(DIRS.len()) {
            let mut flag = true;
            for dir in dirs.0 {
                if map.in_bounds(*elf + dir) && map[*elf + dir] == '#' {
                    flag = false;
                    break;
                }
            }
            if flag {
                *pos = Some(*elf + dirs.1);
                break;
            }
        }
    }

    let mut rev = HashMap::<IVec2, Vec<_>>::new();
    for (elf, pos) in elves.iter() {
        if pos.is_some() {
            rev.entry((*pos)?).or_default().push(*elf);
        }
    }
    let mut flag = false;
    for (pos, elf) in rev.into_iter() {
        if elf.len() == 1 {
            map[*(elf.first()?)] = '.';
            map[pos] = '#';
            flag = true;
        }
    }

    if flag {
        Some(extend(map))
    } else {
        None
    }
}

pub fn part_one(lines: &[String]) -> Option<usize> {
    let mut map = parse_file(lines)?;
    print(&map);

    for skip in 0..10 {
        map = simulate(map, skip)?;
        print(&map);
    }

    let count = map.iter().filter(|chr| **chr == '#').count();
    Some((map.width() - 2) * (map.height() - 2) - count)
}

pub fn part_two(lines: &[String]) -> Option<usize> {
    let mut map = parse_file(lines)?;

    let mut i = 0;
    while let Some(ret) = simulate(map, i) {
        map = ret;
        i += 1;
    }

    Some(i + 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
