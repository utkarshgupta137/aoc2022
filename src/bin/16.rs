use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

use cached::proc_macro::cached;
use itertools::Itertools;

type Valves = HashMap<String, Valve>;
type DistValves = HashMap<String, DistValve>;

const START: &str = "AA";
const TIME1: i32 = 30;
const TIME2: i32 = 26;

#[derive(Clone)]
struct Valve {
    name: String,
    rate: i32,
    tunnels: Vec<String>,
}

struct DistValve {
    name: String,
    rate: i32,
    tunnels: HashMap<String, i32>,
}

impl From<Valve> for DistValve {
    fn from(valve: Valve) -> Self {
        Self {
            name: valve.name,
            rate: valve.rate,
            tunnels: HashMap::new(),
        }
    }
}

impl fmt::Debug for DistValve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn get_min_dist(valves: &Valves, valve: &Valve, goal: &str) -> i32 {
    let mut dists = Vec::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((valve, 0));

    while let Some((valve, dist)) = queue.pop_front() {
        if valve.name == goal {
            dists.push(dist);
            continue;
        }

        for tunnel in &valve.tunnels {
            let valve = valves.get(tunnel).unwrap();
            if !visited.contains(&valve.name) {
                visited.insert(valve.name.clone());
                queue.push_back((valve, dist + 1));
            }
        }
    }
    dists.into_iter().max().unwrap()
}

#[cached(
    key = "String",
    convert = r##"{ format!("{}:{}:{}:{:?}", p2, time, curr.name, to_visit) }"##
)]
fn dfs(
    valves: &DistValves,
    curr: &DistValve,
    time: i32,
    to_visit: HashSet<String>,
    p2: bool,
) -> i32 {
    let mut flows = vec![0];
    for valve in &to_visit {
        let dist = curr.tunnels.get(valve).unwrap();
        let mut cloned = to_visit.clone();
        cloned.remove(valve);
        let valve = valves.get(valve).unwrap();
        if time > *dist {
            let flow =
                valve.rate * (time - dist - 1) + dfs(valves, valve, time - dist - 1, cloned, p2);
            flows.push(flow);
        }
    }
    if p2 {
        flows.push(dfs(
            valves,
            valves.get(START).unwrap(),
            TIME2,
            to_visit,
            false,
        ));
    }
    flows.into_iter().max().unwrap()
}

fn parse_file(lines: &[String]) -> (DistValves, HashSet<String>) {
    let valves = lines
        .iter()
        .map(|line| {
            let (name_rate, tunnels) = line.split_once("; tunnels lead to valves ").unwrap();
            let (name, rate) = name_rate.split_once(" has flow rate=").unwrap();
            let name = name.split_once("Valve ").unwrap().1.to_string();
            let rate = rate.parse::<i32>().unwrap();
            let tunnels = tunnels
                .split(", ")
                .map(|tunnel| tunnel.to_string())
                .collect_vec();
            (
                name.clone(),
                Valve {
                    name,
                    rate,
                    tunnels,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    let mut dist_valves = HashMap::new();
    for valve in valves.values().sorted_by_key(|valve| &valve.name) {
        let mut dist_valve = DistValve::from(valve.clone());
        for tunnel in valves.keys() {
            let rate = valves.get(tunnel).unwrap().rate;
            if rate > 0 {
                dist_valve
                    .tunnels
                    .insert(tunnel.clone(), get_min_dist(&valves, valve, tunnel));
            }
        }
        dist_valves.insert(dist_valve.name.clone(), dist_valve);
    }

    let to_visit = dist_valves
        .get(START)
        .unwrap()
        .tunnels
        .keys()
        .cloned()
        .collect::<HashSet<String>>();

    (dist_valves, to_visit)
}

pub fn part_one(lines: &[String]) -> Option<i32> {
    let (dist_valves, to_visit) = parse_file(lines);
    Some(dfs(
        &dist_valves,
        dist_valves.get(START).unwrap(),
        TIME1,
        to_visit,
        false,
    ))
}

pub fn part_two(lines: &[String]) -> Option<i32> {
    let (dist_valves, to_visit) = parse_file(lines);
    Some(dfs(
        &dist_valves,
        dist_valves.get(START).unwrap(),
        TIME2,
        to_visit,
        true,
    ))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
