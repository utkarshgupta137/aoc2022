use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Int(u32),
    List(Vec<Value>),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Int(int1), Value::Int(int2)) => int1.partial_cmp(int2),
            (Value::Int(int1), Value::List(_)) => {
                Value::List(vec![Value::Int(*int1)]).partial_cmp(other)
            }
            (Value::List(_), Value::Int(int2)) => {
                self.partial_cmp(&Value::List(vec![Value::Int(*int2)]))
            }
            (Value::List(list1), Value::List(list2)) => list1.partial_cmp(list2),
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Value {
    fn push(&mut self, depth: usize, val: Value) {
        if depth > 0 {
            match self {
                Self::Int(_) => unreachable!(),
                Self::List(list) => {
                    list.last_mut().unwrap().push(depth - 1, val);
                }
            }
        } else {
            match self {
                Self::Int(_) => unreachable!(),
                Self::List(list) => {
                    list.push(val);
                }
            }
        }
    }
}

fn parse_file(lines: &[String]) -> Vec<Value> {
    lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            // Hack to handle 10
            let line = line.replace("10", "a");
            let mut row = Value::List(Vec::new());
            let mut depth = 0;
            for char in line[1..line.len() - 1].chars() {
                match char {
                    '[' => {
                        row.push(depth, Value::List(Vec::new()));
                        depth += 1;
                    }
                    ']' => {
                        depth -= 1;
                    }
                    ',' => {}
                    int => {
                        row.push(depth, Value::Int(int.to_digit(11).unwrap()));
                    }
                }
            }
            row
        })
        .collect_vec()
}

pub fn part_one(lines: &[String]) -> Option<usize> {
    let input = parse_file(lines);
    let mut sum = 0;
    for (idx, (left, right)) in input
        .chunks(2)
        .map(|chunk| (chunk[0].clone(), chunk[1].clone()))
        .enumerate()
    {
        if left < right {
            sum += idx + 1;
        }
    }
    Some(sum)
}

pub fn part_two(lines: &[String]) -> Option<usize> {
    let mut input = parse_file(lines);
    let divider1 = Value::List(vec![Value::List(vec![Value::Int(2)])]);
    let divider2 = Value::List(vec![Value::List(vec![Value::Int(6)])]);
    input.push(divider1.clone());
    input.push(divider2.clone());
    input.sort();
    let mut prod = 1;
    for (idx, row) in input.iter().enumerate() {
        if row == &divider1 || row == &divider2 {
            prod *= idx + 1;
        }
    }
    Some(prod)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
