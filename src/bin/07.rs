use std::collections::HashMap;

fn parse_file(lines: &[String]) -> HashMap<String, i32> {
    let mut sizes = HashMap::new();
    let mut path = Vec::new();
    for line in lines {
        if let Some((_, dir)) = line.split_once("$ cd ") {
            if dir == ".." {
                path.pop();
            } else {
                path.push(dir.to_string());
            }
        } else if line != "$ ls" && !line.starts_with("dir ") {
            let (size, _) = line.split_once(' ').unwrap();
            insert(&mut sizes, &path, 0, size.parse::<i32>().unwrap());
        }
    }
    sizes
}

fn insert(sizes: &mut HashMap<String, i32>, path: &Vec<String>, idx: usize, size: i32) {
    if idx < path.len() {
        sizes
            .entry(path[0..=idx].join(" "))
            .and_modify(|v| *v += size)
            .or_insert(size);
        insert(sizes, path, idx + 1, size);
    }
}

pub fn part_one(lines: &[String]) -> Option<i32> {
    let sizes = parse_file(lines);
    let mut total = 0;
    for size in sizes.values() {
        if *size <= 100_000 {
            total += size;
        }
    }
    Some(total)
}

pub fn part_two(lines: &[String]) -> Option<i32> {
    let sizes = parse_file(lines);
    let required = sizes["/"] - 40_000_000;
    let mut curr = sizes["/"];
    for size in sizes.values() {
        if *size >= required && *size < curr {
            curr = *size;
        }
    }
    Some(curr)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
