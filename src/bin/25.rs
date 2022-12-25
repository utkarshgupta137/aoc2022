use itertools::Itertools;

fn parse_file(lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .map(|line| line.chars().collect::<String>())
        .collect_vec()
}

fn chars_to_num(chars: &str) -> i128 {
    let mut num: i128 = 0;
    let mut base = 1;
    for char in chars.chars().rev() {
        let (op, radix) = match char {
            '2' => (1, 2),
            '1' => (1, 1),
            '0' => (1, 0),
            '-' => (-1, 1),
            '=' => (-1, 2),
            _ => unreachable!(),
        };
        num += op * base * radix;
        base *= 5;
    }
    num
}

fn num_to_chars(goal: i128) -> String {
    if goal != 0 {
        let mut idx = (goal + 2) % 5;
        while idx < 0 {
            idx += 5;
        }
        return num_to_chars((goal + 2) / 5) + ["=", "-", "0", "1", "2"][idx as usize];
    }
    "".to_string()
}

pub fn part_one(lines: &[String]) -> Option<String> {
    let nums = parse_file(lines);
    println!("{:?}", nums);
    let sum = nums.iter().map(|chars| chars_to_num(chars)).sum::<i128>();
    println!("{:?}", sum);
    Some(num_to_chars(sum))
}

pub fn part_two(lines: &[String]) -> Option<String> {
    let _ = parse_file(lines);
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some("2=-1=0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_two(&input), None);
    }
}
