use itertools::Itertools;

#[cfg(test)]
const HEIGHT: usize = 10;

#[cfg(not(test))]
const HEIGHT: usize = 165;

const Y: usize = HEIGHT + 2;
const X: usize = 2 * Y + 1;

struct Cave {
    state: [[char; Y]; X],
}

impl Cave {
    #[allow(clippy::needless_range_loop)]
    fn new(rocks: &Vec<Vec<(usize, usize)>>) -> Self {
        // Rocks
        let mut state = [['.'; Y]; X];
        for rock in rocks {
            for line in rock.windows(2) {
                let (mut x1, mut y1) = line[0];
                let (mut x2, mut y2) = line[1];
                if x1 == x2 {
                    if y1 > y2 {
                        (y1, y2) = (y2, y1);
                    }
                    for y in y1..=y2 {
                        state[x1][y] = '#';
                    }
                } else if y1 == y2 {
                    if x1 > x2 {
                        (x1, x2) = (x2, x1);
                    }
                    for x in x1..=x2 {
                        state[x][y1] = '#';
                    }
                } else {
                    unreachable!();
                }
            }
        }

        // Floor
        for x in 0..X {
            state[x][Y - 1] = '#';
        }

        // Source
        state[Y][0] = '+';

        Self { state }
    }

    fn release(&mut self, x: usize, y: usize, p1: bool) -> bool {
        if p1 && y == HEIGHT {
            return false;
        }
        if self.state[x][y + 1] == '.' {
            return self.release(x, y + 1, p1);
        }
        if x > 0 && self.state[x - 1][y + 1] == '.' {
            return self.release(x - 1, y + 1, p1);
        }
        if x + 1 < X && self.state[x + 1][y + 1] == '.' {
            return self.release(x + 1, y + 1, p1);
        }
        if self.state[x][y] == 'o' {
            false
        } else {
            self.state[x][y] = 'o';
            true
        }
    }

    #[cfg(test)]
    fn print(&self) {
        println!();
        for idx in 0..Y {
            println!(
                "{}",
                self.state.iter().map(|col| col[idx]).collect::<String>()
            );
        }
    }
}

fn parse_file(lines: &[String]) -> Vec<Vec<(usize, usize)>> {
    lines
        .iter()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let (x, y) = point.split_once(',').unwrap();
                    (
                        x.parse::<usize>().unwrap() + Y - 500,
                        y.parse::<usize>().unwrap(),
                    )
                })
                .collect_vec()
        })
        .collect_vec()
}

pub fn part_one(lines: &[String]) -> Option<i32> {
    let rocks = parse_file(lines);
    let mut cave = Cave::new(&rocks);

    let mut sand = 0;
    while cave.release(Y, 0, true) {
        sand += 1;
        if sand % 10 == 0 {
            #[cfg(test)]
            cave.print();
        }
    }
    if sand % 10 != 0 {
        #[cfg(test)]
        cave.print();
    }
    Some(sand)
}

pub fn part_two(lines: &[String]) -> Option<i32> {
    let rocks = parse_file(lines);

    let mut cave = Cave::new(&rocks);

    let mut sand = 0;
    while cave.release(Y, 0, false) {
        sand += 1;
        if sand % 1000 == 0 {
            #[cfg(test)]
            cave.print();
        }
    }
    if sand % 1000 != 0 {
        #[cfg(test)]
        cave.print();
    }
    Some(sand)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
