use itertools::Itertools;

fn parse(lines: &[String]) -> Vec<i32> {
    lines
        .split(|line| line.is_empty())
        .map(|foods| {
            foods
                .iter()
                .map(|food| food.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect_vec()
}

pub fn part_one(lines: &[String]) -> Option<i32> {
    let elfs = parse(lines);
    Some(*elfs.iter().max().unwrap())
}

pub fn part_two(lines: &[String]) -> Option<i32> {
    let mut elfs = parse(lines);
    elfs.sort_unstable();
    Some(elfs.iter().rev().take(3).sum::<i32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
