use itertools::Itertools;

#[allow(clippy::type_complexity)]
fn parse_file(lines: &[String]) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let mut split = lines.split(|line| line.is_empty());

    let rows = split.next().unwrap();
    let len = rows
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut cases = vec![Vec::new(); len];
    for row in rows[0..rows.len() - 1].iter() {
        for (i, case) in row
            .chars()
            .collect_vec()
            .chunks(4)
            .map(|row| row[1])
            .enumerate()
            .filter(|(_i, case)| *case != ' ')
        {
            cases[i].insert(0, case);
        }
    }

    let ops = split
        .next()
        .unwrap()
        .iter()
        .map(|line| {
            let mut split = line.split_whitespace();
            (
                split.nth(1).unwrap().parse::<usize>().unwrap(),
                split.nth(1).unwrap().parse::<usize>().unwrap() - 1,
                split.nth(1).unwrap().parse::<usize>().unwrap() - 1,
            )
        })
        .collect_vec();

    (cases, ops)
}

pub fn part_one(lines: &[String]) -> Option<String> {
    let (mut cases, ops) = parse_file(lines);
    for (n, i, j) in ops.iter() {
        let len = cases[*i].len() - n;
        let mut vals = cases[*i].drain(len..).rev().collect();
        cases[*j].append(&mut vals);
    }
    Some(
        cases
            .iter()
            .map(|row| row.last().unwrap())
            .collect::<String>(),
    )
}

pub fn part_two(lines: &[String]) -> Option<String> {
    let (mut cases, ops) = parse_file(lines);
    for (n, i, j) in ops.iter() {
        let len = cases[*i].len() - n;
        let mut vals = cases[*i].drain(len..).collect();
        cases[*j].append(&mut vals);
    }
    Some(
        cases
            .iter()
            .map(|row| row.last().unwrap())
            .collect::<String>(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
