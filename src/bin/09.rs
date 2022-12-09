use std::collections::HashSet;

use itertools::Itertools;

#[cfg(test)]
type Dims = (usize, usize);
type Pos = (i32, i32);

// const SHAPE1: Dims = (6, 5);
// const START1: Pos = (0, 0);

#[cfg(test)]
const SHAPE2: Dims = (26, 21);
const START2: Pos = (11, 5);

#[derive(Debug, Clone, Copy)]
struct Position<const N: usize> {
    head: Pos,
    tails: [Pos; N],
}

impl<const N: usize> Position<N> {
    fn move_head(&mut self, op: &char) {
        match op {
            'R' => self.head.0 += 1,
            'L' => self.head.0 -= 1,
            'U' => self.head.1 += 1,
            'D' => self.head.1 -= 1,
            _ => unreachable!(),
        };
    }

    fn move_knot(head: &Pos, tail: &mut Pos) {
        let (hx, hy) = head;
        let (tx, ty) = tail;
        let dx = hx.abs_diff(*tx);
        let dy = hy.abs_diff(*ty);
        if dx > 1 {
            if hx > tx {
                *tx += 1;
            } else {
                *tx -= 1;
            }
            if dy > 0 {
                if hy > ty {
                    *ty += 1;
                } else {
                    *ty -= 1;
                }
            }
        } else if dy > 1 {
            if hy > ty {
                *ty += 1;
            } else {
                *ty -= 1;
            }
            if dx > 0 {
                if hx > tx {
                    *tx += 1;
                } else {
                    *tx -= 1;
                }
            }
        }
    }
}

#[cfg(test)]
struct Bridge<const R: usize, const C: usize> {
    state: [[char; R]; C],
    start: Dims,
}

#[cfg(test)]
impl<const R: usize, const C: usize> Bridge<R, C> {
    fn new(start: Pos) -> Self {
        Bridge {
            state: [['.'; R]; C],
            start: (start.0 as usize, start.1 as usize),
        }
    }

    fn _set_pos(&mut self, pos: &Pos, knot: char) {
        self.state[pos.1 as usize][pos.0 as usize] = knot;
    }

    fn print<const N: usize>(&mut self, pos: &Position<N>) {
        println!();
        self.state = [['.'; R]; C];
        self.state[self.start.1][self.start.0] = 's';

        if pos.tails.len() == 1 {
            self._set_pos(&pos.tails[0], 'T');
        } else {
            for (i, tail) in pos.tails.iter().rev().enumerate() {
                self._set_pos(tail, (i + 1).to_string().chars().next().unwrap());
            }
        }
        self._set_pos(&pos.head, 'H');

        for row in self.state.iter().rev() {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

fn parse_file(lines: &[String]) -> Vec<(char, i32)> {
    lines
        .iter()
        .map(|line| {
            let (dir, dist) = line.split_once(' ').unwrap();
            (dir.chars().next().unwrap(), dist.parse::<i32>().unwrap())
        })
        .collect_vec()
}

pub fn part_one(lines: &[String]) -> Option<usize> {
    let ops = parse_file(lines);

    let mut pos = Position {
        head: START2,
        tails: [START2; 1],
    };
    #[cfg(test)]
    let mut bridge = Bridge::<{ SHAPE2.0 }, { SHAPE2.1 }>::new(START2);

    let mut visited = HashSet::new();
    for (op, dist) in ops {
        for _ in 0..dist {
            pos.move_head(&op);
            let mut prev = &pos.head;
            for curr in pos.tails.iter_mut() {
                Position::<1>::move_knot(prev, curr);
                prev = curr;
            }
            visited.insert(*pos.tails.last().unwrap());
            #[cfg(test)]
            bridge.print(&pos);
        }
    }
    Some(visited.len())
}

pub fn part_two(lines: &[String]) -> Option<usize> {
    let ops = parse_file(lines);

    let mut pos = Position {
        head: START2,
        tails: [START2; 9],
    };
    #[cfg(test)]
    let mut bridge = Bridge::<{ SHAPE2.0 }, { SHAPE2.1 }>::new(START2);

    let mut visited = HashSet::new();
    for (op, dist) in ops {
        for _ in 0..dist {
            pos.move_head(&op);
            let mut prev = &pos.head;
            for curr in pos.tails.iter_mut() {
                Position::<9>::move_knot(prev, curr);
                prev = curr;
            }
            visited.insert(*pos.tails.last().unwrap());
        }
        #[cfg(test)]
        bridge.print(&pos);
    }
    Some(visited.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
