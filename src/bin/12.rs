use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

type Dims = (usize, usize);
type Map<const R: usize, const C: usize> = [[char; C]; R];

#[cfg(test)]
const SHAPE: (usize, usize) = (5, 8);
#[cfg(not(test))]
const SHAPE: (usize, usize) = (41, 161);

fn read_map<const R: usize, const C: usize>(lines: &[String]) -> (Map<R, C>, Dims, Dims) {
    let mut map = [['.'; C]; R];
    let mut start = (0, 0);
    let mut end = (0, 0);
    lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            for (j, mut char) in line.chars().enumerate() {
                if char == 'S' {
                    start = (i, j);
                    char = 'a';
                } else if char == 'E' {
                    end = (i, j);
                    char = 'z';
                }
                map[i][j] = char;
            }
        })
        .collect_vec();
    (map, start, end)
}

pub fn part_one(lines: &[String]) -> Option<usize> {
    let (map, start, end) = read_map::<{ SHAPE.0 }, { SHAPE.1 }>(lines);

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((pos, dist)) = queue.pop_front() {
        if pos == end {
            return Some(dist);
        }
        let (x, y) = pos;
        let curr = map[x][y];
        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if x as isize + dx < 0
                || x as isize + dx >= SHAPE.0 as isize
                || y as isize + dy < 0
                || y as isize + dy >= SHAPE.1 as isize
            {
                continue;
            }
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;
            let npos = (nx, ny);
            let next = map[nx][ny];
            if !visited.contains(&npos) && (curr as usize + 1) >= (next as usize) {
                visited.insert(npos);
                queue.push_back((npos, dist + 1));
            }
        }
    }
    None
}

pub fn part_two(lines: &[String]) -> Option<usize> {
    let (map, _, start) = read_map::<{ SHAPE.0 }, { SHAPE.1 }>(lines);
    let end = 'a';

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((pos, dist)) = queue.pop_front() {
        let (x, y) = pos;
        let curr = map[x][y];
        if curr == end {
            return Some(dist);
        }
        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if x as isize + dx < 0
                || x as isize + dx >= SHAPE.0 as isize
                || y as isize + dy < 0
                || y as isize + dy >= SHAPE.1 as isize
            {
                continue;
            }
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;
            let npos = (nx, ny);
            let next = map[nx][ny];
            if !visited.contains(&npos) && (next as usize + 1) >= (curr as usize) {
                visited.insert(npos);
                queue.push_back((npos, dist + 1));
            }
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
