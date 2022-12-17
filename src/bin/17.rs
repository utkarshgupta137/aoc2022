use itertools::Itertools;

type Row = [char; 7];

const NUM1: usize = 2022;
const NUM2: usize = 1_000_000_000_000;

const SHAPE0: [Row; 1] = [['.', '.', '#', '#', '#', '#', '.']];
const SHAPE1: [Row; 3] = [
    ['.', '.', '.', '#', '.', '.', '.'],
    ['.', '.', '#', '#', '#', '.', '.'],
    ['.', '.', '.', '#', '.', '.', '.'],
];
const SHAPE2: [Row; 3] = [
    ['.', '.', '.', '.', '#', '.', '.'],
    ['.', '.', '.', '.', '#', '.', '.'],
    ['.', '.', '#', '#', '#', '.', '.'],
];
const SHAPE3: [Row; 4] = [
    ['.', '.', '#', '.', '.', '.', '.'],
    ['.', '.', '#', '.', '.', '.', '.'],
    ['.', '.', '#', '.', '.', '.', '.'],
    ['.', '.', '#', '.', '.', '.', '.'],
];
const SHAPE4: [Row; 2] = [
    ['.', '.', '#', '#', '.', '.', '.'],
    ['.', '.', '#', '#', '.', '.', '.'],
];

#[derive(Default)]
struct Tetris {
    state: Vec<Row>,
    curr: u8,
    jets: Vec<i8>,
    idx: usize,
}

impl Tetris {
    fn get_height(&self) -> usize {
        let mut height = 0;
        for (idx, row) in self.state.iter().enumerate().rev() {
            if row.contains(&'#') {
                height = idx + 1;
                break;
            }
        }
        height
    }

    fn shape_shift(shape: &[Row], mut x: i32) -> Vec<Row> {
        let mut shape = shape.to_owned();
        while x != 2 {
            if x < 2 {
                for row in shape.iter_mut() {
                    *row = [row[1], row[2], row[3], row[4], row[5], row[6], '.'];
                }
                x += 1;
            } else {
                for row in shape.iter_mut() {
                    *row = ['.', row[0], row[1], row[2], row[3], row[4], row[5]];
                }
                x -= 1;
            }
        }
        shape
    }

    fn placable(state: &[Row], shape: &[Row], x: i32, y: usize) -> bool {
        let shape = Tetris::shape_shift(shape, x);
        for (idx, row) in shape.iter().rev().enumerate() {
            for (a, b) in state[y + idx].iter().zip(row.iter()) {
                if *a == '#' && *b == '#' {
                    return false;
                }
            }
        }
        true
    }

    #[allow(clippy::comparison_chain)]
    #[allow(clippy::collapsible_if)]
    fn spawn(&mut self) {
        let Self {
            state,
            curr,
            jets,
            idx,
        } = self;

        let mut height = 0;
        for (idx, row) in state.iter().enumerate().rev() {
            if row.contains(&'#') {
                height = idx + 1;
                break;
            }
        }
        while state.len() < height + 7 {
            state.push(['.'; 7]);
        }
        let (shape, (_sh, sw)) = match curr {
            0 => (SHAPE0.to_vec(), (1, 4)),
            1 => (SHAPE1.to_vec(), (3, 3)),
            2 => (SHAPE2.to_vec(), (3, 3)),
            3 => (SHAPE3.to_vec(), (4, 1)),
            4 => (SHAPE4.to_vec(), (2, 2)),
            _ => unreachable!(),
        };

        let mut x = 2;
        let mut y = height + 3;
        for jet in jets.iter().cycle().skip(*idx) {
            if *jet < 0 {
                if x > 0 && Tetris::placable(state, &shape, x - 1, y) {
                    x -= 1;
                }
            } else if *jet > 0 {
                if x + sw < 7 && Tetris::placable(state, &shape, x + 1, y) {
                    x += 1;
                }
            }
            *idx += 1;
            if y > 0 && Tetris::placable(state, &shape, x, y - 1) {
                y -= 1;
            } else {
                break;
            }
        }

        let shape = Tetris::shape_shift(&shape, x);
        for (i, row) in shape.iter().rev().enumerate() {
            for (j, char) in row.iter().enumerate() {
                if *char == '#' {
                    state[y + i][j] = *char;
                }
            }
        }

        *curr += 1;
        if *curr == 5 {
            *curr = 0;
        }
    }

    #[cfg(test)]
    fn print(&self) {
        for row in self.state.iter().rev() {
            println!("{}", row.iter().collect::<String>());
        }
        println!();
    }
}

pub fn part_one(lines: &[String]) -> Option<usize> {
    let jets = lines
        .first()?
        .chars()
        .map(|char| match char {
            '<' => -1,
            '>' => 1,
            _ => unreachable!(),
        })
        .collect_vec();
    let mut tetris = Tetris {
        jets,
        ..Default::default()
    };

    for _ in 0..10 {
        tetris.spawn();
        #[cfg(test)]
        tetris.print();
    }
    for _ in 10..NUM1 {
        tetris.spawn();
    }
    Some(tetris.get_height())
}

pub fn part_two(lines: &[String]) -> Option<usize> {
    let jets = lines
        .first()?
        .chars()
        .map(|char| match char {
            '<' => -1,
            '>' => 1,
            _ => unreachable!(),
        })
        .collect_vec();
    let mut tetris = Tetris {
        jets,
        ..Default::default()
    };

    // Get height increments
    let mut heights = Vec::new();
    let mut prev = 0;
    for idx in 0..10 * NUM1 {
        tetris.spawn();
        let height = if idx != 0 { tetris.get_height() } else { 0 };
        heights.push(height - prev);
        prev = height;
    }

    // Find pattern
    let heights_str = heights
        .iter()
        .map(|height| char::from_digit(*height as u32, 10).unwrap())
        .collect::<String>();
    let mut pattern = None;
    'outer: for offset in 0..10 * NUM1 {
        for len in 10..(heights_str.len() - offset) {
            if heights_str[offset + len..].contains(&heights_str[offset..offset + len]) {
                pattern = Some((offset, len));
                if heights_str[offset + len..].find(&heights_str[offset..offset + len])? == 0 {
                    break 'outer;
                }
            } else {
                break;
            }
        }
    }

    // Calculate height based on pattern
    let (offset, len) = pattern?;
    let base = heights[..offset].iter().sum::<usize>();
    let mid = ((NUM2 - offset) / len) * heights[offset..offset + len].iter().sum::<usize>();
    let rem = (NUM2 - offset) % len;
    let end = heights[offset..offset + rem].iter().sum::<usize>();
    Some(base + mid + end)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        println!();
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        println!();
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
