use itertools::Itertools;

type Display = [[char; 40]; 6];

#[derive(Debug)]
enum Instruction {
    NoOp,
    AddX(i32),
}

impl From<&String> for Instruction {
    fn from(inp: &String) -> Self {
        if inp == "noop" {
            return Self::NoOp;
        }

        let (cmd, val) = inp.split_once(' ').unwrap();
        match cmd {
            "addx" => Self::AddX(val.parse::<i32>().unwrap()),
            _ => unimplemented!(),
        }
    }
}

fn parse_file(lines: &[String]) -> Vec<(i32, i32)> {
    let ops = lines.iter().map(Instruction::from).collect_vec();

    let mut cycles = Vec::new();
    let mut cycle = 0;
    let mut x = 1;
    for op in ops {
        match op {
            Instruction::NoOp => {
                cycle += 1;
                cycles.push((cycle, x));
            }
            Instruction::AddX(val) => {
                cycle += 1;
                cycles.push((cycle, x));
                cycle += 1;
                cycles.push((cycle, x));
                x += val;
            }
        };
    }
    cycles
}

fn print_display(disp: Display) {
    for row in disp {
        println!("{}", row.into_iter().collect::<String>());
    }
}

pub fn part_one(lines: &[String]) -> Option<i32> {
    let cycles = parse_file(lines);

    let res = cycles
        .iter()
        .filter_map(|(cycle, x)| {
            if *cycle == 20 || (*cycle + 20) % 40 == 0 {
                Some((cycle, cycle * x))
            } else {
                None
            }
        })
        .collect_vec();
    Some(res.iter().map(|(_cycle, x)| x).sum::<i32>())
}

pub fn part_two(lines: &[String]) -> Option<String> {
    let cycles = parse_file(lines);
    let mut display: Display = [['.'; 40]; 6];
    for (cycle, x) in cycles {
        let i = ((cycle - 1) / 40) as usize;
        let j = ((cycle - 1) % 40) as usize;
        if x.abs_diff(j as i32) < 2 {
            display[i][j] = '#';
        }
    }
    print_display(display);
    Some(String::new())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
