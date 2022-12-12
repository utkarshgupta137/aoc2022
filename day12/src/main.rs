use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

type Dims = (usize, usize);
type Map<const R: usize, const C: usize> = [[char; C]; R];

const SAMPLE: (&str, usize, usize) = ("sample.txt", 5, 8);
const INPUT: (&str, usize, usize) = ("input.txt", 41, 161);

fn read_map<const R: usize, const C: usize>(file: &str) -> (Map<R, C>, Dims, Dims) {
    let mut map = [['.'; C]; R];
    let mut start = (0, 0);
    let mut end = (0, 0);
    BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|line| line.unwrap())
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

fn bfs<const R: usize, const C: usize>(map: &Map<R, C>, start: Dims, end: Dims) -> Option<usize> {
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
                || x as isize + dx >= R as isize
                || y as isize + dy < 0
                || y as isize + dy >= C as isize
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

fn bfs_rev<const R: usize, const C: usize>(
    map: &Map<R, C>,
    start: Dims,
    end: char,
) -> Option<usize> {
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
                || x as isize + dx >= R as isize
                || y as isize + dy < 0
                || y as isize + dy >= C as isize
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
    let (map, start, end) = read_map::<{ INPUT.1 }, { INPUT.2 }>(INPUT.0);

    // Part 1
    println!("{:?}", bfs(&map, start, end));

    // Part 2
    println!("{:?}", bfs_rev(&map, end, 'a'));
}
