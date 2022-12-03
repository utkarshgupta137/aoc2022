use std::collections::HashSet;

use itertools::Itertools;

fn get_priority(c: char) -> u32 {
    let r = c.to_digit(36).unwrap() - 9;
    if c.is_uppercase() {
        r + 26
    } else {
        r
    }
}

pub fn part_one(lines: &[String]) -> Option<u32> {
    let rucksacks = lines.iter().map(|line| {
        let (c1, c2) = line.split_at(line.len() / 2);
        let h1: HashSet<char> = HashSet::from_iter(c1.chars());
        let h2: HashSet<char> = HashSet::from_iter(c2.chars());
        let c = h1.intersection(&h2).cloned().collect_vec()[0];
        get_priority(c)
    });
    Some(rucksacks.sum::<u32>())
}

pub fn part_two(lines: &[String]) -> Option<u32> {
    let badges = lines.chunks(3).map(|group| {
        let mut group = group.iter().map(|rucksack| {
            let h: HashSet<char> = HashSet::from_iter(rucksack.chars());
            h
        });
        let h = group.next().unwrap();
        let c = group.fold(h, |acc, h| acc.intersection(&h).cloned().collect());
        get_priority(c.into_iter().next().unwrap())
    });
    Some(badges.sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
