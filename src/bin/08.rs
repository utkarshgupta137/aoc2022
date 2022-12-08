use std::collections::HashSet;

use itertools::Itertools;

fn parse_file(lines: &[String]) -> Vec<Vec<u32>> {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect_vec()
        })
        .collect_vec()
}

pub fn part_one(lines: &[String]) -> Option<usize> {
    let trees = parse_file(lines);
    let len = trees.len();
    let mut visible = HashSet::new();
    for (i, row) in trees.iter().enumerate().take(len - 1).skip(1) {
        for (j, tree) in row.iter().enumerate().take(len - 1).skip(1) {
            if tree > row[..j].iter().max().unwrap()
                || tree > row[j + 1..].iter().max().unwrap()
                || *tree > trees[..i].iter().map(|row| row[j]).max().unwrap()
                || *tree > trees[i + 1..].iter().map(|row| row[j]).max().unwrap()
            {
                visible.insert((i, j, tree));
            }
        }
    }
    Some(visible.len() + (len - 1) * 4)
}

pub fn part_two(lines: &[String]) -> Option<usize> {
    let trees = parse_file(lines);
    let len = trees.len();
    let mut best = 0;
    for (i, row) in trees.iter().enumerate().take(len - 1).skip(1) {
        for (j, tree) in row.iter().enumerate().take(len - 1).skip(1) {
            let mut scores = (0, 0, 0, 0);
            for nb in row[..j].iter().rev() {
                scores.0 += 1;
                if nb >= tree {
                    break;
                }
            }
            for nb in row[j + 1..].iter() {
                scores.1 += 1;
                if nb >= tree {
                    break;
                }
            }
            for nb in trees[..i].iter().map(|row| row[j]).rev() {
                scores.2 += 1;
                if nb >= *tree {
                    break;
                }
            }
            for nb in trees[i + 1..].iter().map(|row| row[j]) {
                scores.3 += 1;
                if nb >= *tree {
                    break;
                }
            }
            let score = scores.0 * scores.1 * scores.2 * scores.3;
            if score > best {
                best = score;
            }
        }
    }
    Some(best)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
