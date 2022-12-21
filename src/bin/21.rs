use std::collections::HashMap;

use regex::Regex;

type Nums = HashMap<String, Num>;

#[derive(Debug, Clone)]
enum Num {
    Int(i64),
    Op(String, char, String),
}

impl Num {
    fn get_val(&self, nums: &Nums) -> Option<i64> {
        match self {
            Self::Int(val) => Some(*val),
            Self::Op(left, op, right) => {
                let left = nums.get(left)?.get_val(nums)?;
                let right = nums.get(right)?.get_val(nums)?;
                let val = match op {
                    '+' => left + right,
                    '-' => left - right,
                    '*' => left * right,
                    '/' => left / right,
                    _ => unreachable!(),
                };
                Some(val)
            }
        }
    }
}

fn parse_file(lines: &[String], p2: bool) -> Nums {
    let re_int = Regex::new(r"(\w+): (\d+)").unwrap();
    let re_op = Regex::new(r"(\w+): (\w+) (.) (\w+)").unwrap();
    lines
        .iter()
        .filter_map(|line| {
            if re_int.is_match(line) {
                let captures = re_int.captures(line)?;
                let mut iter = captures.iter();
                iter.next();
                let name = iter.next()??.as_str().to_string();
                let val = iter.next()??.as_str().parse::<i64>().ok()?;
                if p2 && name == "humn" {
                    None
                } else {
                    Some((name, Num::Int(val)))
                }
            } else if re_op.is_match(line) {
                let captures = re_op.captures(line)?;
                let mut iter = captures.iter();
                iter.next();
                let name = iter.next()??.as_str().to_string();
                let left = iter.next()??.as_str().to_string();
                let op = iter.next()??.as_str().chars().next().unwrap();
                let right = iter.next()??.as_str().to_string();
                if p2 && name == "root" {
                    Some((name, Num::Op(left, '-', right)))
                } else {
                    Some((name, Num::Op(left, op, right)))
                }
            } else {
                unreachable!();
            }
        })
        .collect()
}

fn solve(nums: &Nums, num: &Num, val: i64) -> Option<i64> {
    let (left, op, right) = match num {
        Num::Op(left, op, right) => (left, op, right),
        _ => unreachable!(),
    };

    if let Some(left) = nums.get(left) {
        if let Some(left) = left.get_val(nums) {
            let val = match op {
                '+' => val - left,
                '-' => left - val,
                '*' => val / left,
                '/' => left / val,
                _ => unreachable!(),
            };
            if let Some(num) = nums.get(right) {
                return solve(nums, num, val);
            }
            return Some(val);
        }
    }
    if let Some(right) = nums.get(right) {
        if let Some(right) = right.get_val(nums) {
            let val = match op {
                '+' => val - right,
                '-' => val + right,
                '*' => val / right,
                '/' => val * right,
                _ => unreachable!(),
            };
            if let Some(num) = nums.get(left) {
                return solve(nums, num, val);
            }
            return Some(val);
        }
    }
    unreachable!()
}

pub fn part_one(lines: &[String]) -> Option<i64> {
    let nums = parse_file(lines, false);
    nums.get("root")?.get_val(&nums)
}

pub fn part_two(lines: &[String]) -> Option<i64> {
    let nums = parse_file(lines, true);
    solve(&nums, nums.get("root")?, 0)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
