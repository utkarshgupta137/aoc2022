use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

type Display = [[char; 40]; 6];

#[derive(Debug)]
enum Instruction {
    NoOp,
    AddX(i32),
}

impl From<String> for Instruction {
    fn from(inp: String) -> Self {
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

fn print_display(disp: Display) {
    for row in disp {
        println!("{}", row.into_iter().collect::<String>());
    }
}

fn main() {
    let ops = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(Instruction::from)
        .collect_vec();

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

    // Part 1
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
    println!("{:?}", res);
    println!("{:?}", res.iter().map(|(_cycle, x)| x).sum::<i32>());

    // Part 2
    let mut display: Display = [['.'; 40]; 6];
    for (cycle, x) in cycles {
        let i = ((cycle - 1) / 40) as usize;
        let j = ((cycle - 1) % 40) as usize;
        if x.abs_diff(j as i32) < 2 {
            display[i][j] = '#';
        }
    }
    print_display(display);
}
