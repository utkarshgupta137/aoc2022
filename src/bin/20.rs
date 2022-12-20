use itertools::Itertools;

type File = Vec<(usize, isize)>;

const DECRYPTION_KEY: isize = 811589153;

fn parse_file(lines: &[String]) -> File {
    lines
        .iter()
        .map(|line| line.parse::<isize>().unwrap())
        .enumerate()
        .collect_vec()
}

fn decrypt(file: &File, circular: &mut File) -> Option<()> {
    let max_idx = file.len() as isize - 1;
    for (key, value) in file {
        let idx = circular.iter().position(|(k, _)| k == key)?;
        let mut new_idx = idx as isize + value;
        new_idx %= max_idx;
        if *value < 0 {
            new_idx += max_idx;
        };
        circular.remove(idx);
        circular.insert(new_idx as usize, (*key, *value));
    }
    None
}

fn get_grove_coordinates_sum(circular: &File) -> Option<isize> {
    let pos = circular.iter().position(|(_, v)| *v == 0)?;
    let p1000 = circular.iter().cycle().nth(pos + 1000)?.1;
    let p2000 = circular.iter().cycle().nth(pos + 2000)?.1;
    let p3000 = circular.iter().cycle().nth(pos + 3000)?.1;
    Some(p1000 + p2000 + p3000)
}

pub fn part_one(lines: &[String]) -> Option<isize> {
    let file = parse_file(lines);

    let mut circular = file.clone();
    decrypt(&file, &mut circular);
    get_grove_coordinates_sum(&circular)
}

pub fn part_two(lines: &[String]) -> Option<isize> {
    let file = parse_file(lines)
        .into_iter()
        .map(|(k, v)| (k, v * DECRYPTION_KEY))
        .collect_vec();

    let mut circular = file.clone();
    for _ in 0..10 {
        decrypt(&file, &mut circular);
    }
    get_grove_coordinates_sum(&circular)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
