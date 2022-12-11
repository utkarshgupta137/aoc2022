use std::cell::RefCell;

use itertools::Itertools;

struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> usize>,
    cond: u64,
}

fn parse_file(lines: &[String]) -> Vec<RefCell<Monkey>> {
    lines
        .iter()
        .chunks(7)
        .into_iter()
        .map(|mut lines| {
            lines.next();

            let items = lines
                .next()
                .unwrap()
                .split_once("Starting items: ")
                .unwrap()
                .1
                .split(", ")
                .map(|item| item.parse::<u64>().unwrap())
                .collect_vec();

            let operation = create_operation(
                lines
                    .next()
                    .unwrap()
                    .split_once("Operation: ")
                    .unwrap()
                    .1
                    .split_whitespace()
                    .map(|op| op.to_string())
                    .collect_vec(),
            );

            let cond = lines
                .next()
                .unwrap()
                .split_once("Test: divisible by ")
                .unwrap()
                .1
                .parse::<u64>()
                .unwrap();

            let if_true = lines
                .next()
                .unwrap()
                .split_once("If true: throw to monkey ")
                .unwrap()
                .1
                .parse::<usize>()
                .unwrap();

            let if_false = lines
                .next()
                .unwrap()
                .split_once("If false: throw to monkey ")
                .unwrap()
                .1
                .parse::<usize>()
                .unwrap();
            let test = create_test(cond, if_true, if_false);

            RefCell::new(Monkey {
                items,
                operation,
                test,
                cond,
            })
        })
        .collect_vec()
}

fn create_operation(inp: Vec<String>) -> Box<dyn Fn(u64) -> u64> {
    if let Ok(val) = inp.get(4).unwrap().parse::<u64>() {
        match inp.get(3).unwrap().as_str() {
            "+" => Box::new(move |inp| inp + val),
            "*" => Box::new(move |inp| inp * val),
            _ => unreachable!(),
        }
    } else {
        match inp.get(3).unwrap().as_str() {
            "+" => Box::new(|inp| inp + inp),
            "*" => Box::new(|inp| inp * inp),
            _ => unreachable!(),
        }
    }
}

fn create_test(cond: u64, if_true: usize, if_false: usize) -> Box<dyn Fn(u64) -> usize> {
    Box::new(move |inp| if inp % cond == 0 { if_true } else { if_false })
}

#[allow(unused_variables)]
pub fn part_one(lines: &[String]) -> Option<i64> {
    let monkeys = parse_file(lines);
    let mut inspected = vec![0; monkeys.len()];
    for round in 1..=20 {
        for (i, monkey) in monkeys.iter().enumerate() {
            for item in monkey.borrow().items.iter() {
                inspected[i] += 1;
                let val = (monkey.borrow().operation)(*item) / 3;
                let idx = (monkey.borrow().test)(val);
                monkeys.get(idx).unwrap().borrow_mut().items.push(val);
            }
            monkey.borrow_mut().items.clear();
        }

        #[cfg(test)]
        println!("Round: {}", round);
        #[cfg(test)]
        for (i, monkey) in monkeys.iter().enumerate() {
            println!("{}: {:?}", i, monkey.borrow().items);
        }
    }

    let mut iter = inspected.into_iter().sorted_unstable().rev();
    Some(iter.next().unwrap() * iter.next().unwrap())
}

#[allow(unused_variables)]
pub fn part_two(lines: &[String]) -> Option<i64> {
    let monkeys = parse_file(lines);
    let mut cond = 1;
    for monkey in monkeys.iter() {
        cond *= monkey.borrow().cond;
    }

    let mut inspected = vec![0; monkeys.len()];
    for round in 1..=10000 {
        for (i, monkey) in monkeys.iter().enumerate() {
            for item in monkey.borrow().items.iter() {
                inspected[i] += 1;
                let val = (monkey.borrow().operation)(*item) % cond;
                let idx = (monkey.borrow().test)(val);
                monkeys.get(idx).unwrap().borrow_mut().items.push(val);
            }
            monkey.borrow_mut().items.clear();
        }

        #[cfg(test)]
        if round == 1 || round == 20 || round % 1000 == 0 {
            println!("{}: {:?}", round, inspected);
        }
    }

    let mut iter = inspected.into_iter().sorted_unstable().rev();
    Some(iter.next().unwrap() * iter.next().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
