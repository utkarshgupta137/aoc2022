use std::cmp::Ordering;

fn parse(line: &str) -> (i32, i32) {
    let (i, j) = line.split_once('-').unwrap();
    (i.parse::<i32>().unwrap(), j.parse::<i32>().unwrap())
}

pub fn part_one(lines: &[String]) -> Option<i32> {
    let sections = lines.iter().map(|line| {
        let (s1, s2) = line.split_once(',').unwrap();
        let (i1, j1) = parse(s1);
        let (i2, j2) = parse(s2);

        (match i1.cmp(&i2) {
            Ordering::Greater => j1 <= j2,
            Ordering::Less => j1 >= j2,
            Ordering::Equal => true,
        }) as i32
    });
    Some(sections.sum::<i32>())
}

pub fn part_two(lines: &[String]) -> Option<i32> {
    let sections = lines.iter().map(|line| {
        let (s1, s2) = line.split_once(',').unwrap();
        let (i1, j1) = parse(s1);
        let (i2, j2) = parse(s2);

        (if i1 > j2 { false } else { i2 <= j1 }) as i32
    });
    Some(sections.sum::<i32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
