use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::read_input;

pub fn part1() {
    let lines = read_input("inputs/day8.txt");

    let commands = lines.clone().into_iter().take(1).collect_vec();
    let lines = lines
        .into_iter()
        .skip(2)
        .map(|x| {
            let (current, dest) = x.split_once(" = ").unwrap();
            let (left, right) = dest.split_once(", ").unwrap();
            let left = left.chars().skip(1).collect::<String>();
            let right = right.chars().take(3).collect::<String>();
            (current.to_string(), left, right)
        })
        .fold(HashMap::new(), |mut acc, (current, left, right)| {
            acc.insert(current, (left, right));
            acc
        });

    let mut pos = "AAA";
    let mut idx = 0;
    let mut command_idx = 0;
    loop {
        let c = commands[0].chars().nth(command_idx).unwrap();
        idx += 1;
        match c {
            'R' => {
                pos = lines.get(pos).unwrap().1.as_str();
            }
            'L' => {
                pos = lines.get(pos).unwrap().0.as_str();
            }
            _ => {}
        }
        if pos == "ZZZ" {
            println!("Day 8 Part 1: {}", idx);
            return;
        }
        command_idx += 1;
        if command_idx >= commands[0].len() {
            command_idx = 0;
        }
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn lcm_of_vec(numbers: &[u64]) -> u64 {
    numbers.iter().fold(1, |acc, &num| lcm(acc, num))
}

pub fn part2() {
    let lines = read_input("inputs/day8.txt");

    let commands = lines.clone().into_iter().take(1).collect_vec();
    let lines = lines
        .into_iter()
        .skip(2)
        .map(|x| {
            let (current, dest) = x.split_once(" = ").unwrap();
            let (left, right) = dest.split_once(", ").unwrap();
            let left = left.chars().skip(1).collect::<String>();
            let right = right.chars().take(3).collect::<String>();
            (current.to_string(), left, right)
        })
        .fold(HashMap::new(), |mut acc, (current, left, right)| {
            acc.insert(current, (left, right));
            acc
        });

    let mut starting_positions: HashSet<String> =
        HashSet::from_iter(lines.keys().cloned().filter(|x| x.ends_with("A")));

    let mut position_denominators: HashMap<String, u64> = HashMap::new();

    for start_pos in starting_positions.into_iter() {
        let mut pos = start_pos.clone();
        let mut idx = 0;
        let mut command_idx = 0;
        loop {
            let c = commands[0].chars().nth(command_idx).unwrap();
            idx += 1;
            match c {
                'R' => {
                    pos = lines.get(&pos).unwrap().1.to_string();
                }
                'L' => {
                    pos = lines.get(&pos).unwrap().0.to_string();
                }
                _ => {}
            }

            if pos.ends_with("Z") {
                position_denominators.insert(start_pos.clone(), idx);
                break;
            }

            command_idx += 1;
            if command_idx >= commands[0].len() {
                command_idx = 0;
            }
        }
    }

    let denominators = position_denominators.values().cloned().collect_vec();
    let lcm = lcm_of_vec(&denominators);
    println!("Day 8 Part 2: {}", lcm);
}
