use std::collections::HashSet;

use itertools::Itertools;

pub fn part_one(lines: &[String]) -> Option<i32> {
    let chars = lines.first().unwrap().chars().collect_vec();
    for (i, window) in chars.windows(4).enumerate() {
        let set: HashSet<&char> = HashSet::from_iter(window);
        if set.len() == 4 {
            return Some((i + 4) as i32);
        }
    }
    None
}

pub fn part_two(lines: &[String]) -> Option<i32> {
    let chars = lines.first().unwrap().chars().collect_vec();
    for (i, window) in chars.windows(14).enumerate() {
        let set: HashSet<&char> = HashSet::from_iter(window);
        if set.len() == 14 {
            return Some((i + 14) as i32);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
