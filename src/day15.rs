use std::str::FromStr;

use itertools::Itertools;

use crate::utils::read_input;

fn hash(str: &str) -> u64 {
    let mut val: u64 = 0;
    for c in str.chars() {
        let ascii = c as u8;
        val += ascii as u64;
        val *= 17;
        val %= 256;
    }
    val
}

pub fn part1() {
    let sum = read_input("inputs/day15.txt")
        .into_iter()
        .flat_map(|x| x.split(',').map(|x| x.to_string()).collect_vec())
        .map(|x| hash(&x))
        .sum::<u64>();

    println!("Day 15, Part 1: {}", sum);
}

enum Operation {
    Remove,
    Insert(u64),
}

struct Command {
    label: String,
    box_idx: usize,
    operation: Operation,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with('-') {
            let label = s[..s.len() - 1].to_string();
            let box_idx = hash(&label) as usize;
            Ok(Command {
                label,
                operation: Operation::Remove,
                box_idx,
            })
        } else {
            let (label, val) = s.split_once('=').unwrap();
            let h = hash(label);

            Ok(Command {
                label: label.to_owned(),
                operation: Operation::Insert(val.parse().unwrap()),
                box_idx: h as usize,
            })
        }
    }
}

fn print_data(data: &Vec<Vec<(String, u64)>>) {
    for (i, d) in data.iter().enumerate() {
        if d.is_empty() {
            continue;
        }
        println!("Box {}: {:?}", i + 1, d);
    }
}

pub fn part2() {
    let mut data: Vec<Vec<(String, u64)>> = vec![];
    for i in 0..256 {
        data.push(vec![]);
    }

    let commands = read_input("inputs/day15.txt")
        .into_iter()
        .flat_map(|x| x.split(',').map(|x| x.to_string()).collect_vec())
        .map(|x| x.parse::<Command>().unwrap())
        .collect_vec();

    for c in commands {
        match c.operation {
            Operation::Insert(val) => {
                if data[c.box_idx].iter().any(|x| x.0 == c.label) {
                    data.get_mut(c.box_idx)
                        .unwrap()
                        .iter_mut()
                        .filter(|x| x.0 == c.label)
                        .for_each(|x| x.1 = val);
                } else {
                    data.get_mut(c.box_idx).unwrap().push((c.label, val));
                }
            }
            Operation::Remove => {
                data.get_mut(c.box_idx).unwrap().retain(|x| x.0 != c.label);
            }
        }
    }

    let total_value = data
        .into_iter()
        .enumerate()
        .map(|(idx, boxx)| {
            let value = boxx
                .iter()
                .enumerate()
                .map(|(box_pos, focal_value)| (box_pos + 1) as u64 * focal_value.1)
                .sum::<u64>();
            value * (idx + 1) as u64
        })
        .sum::<u64>();

    println!("Day 15, Part 2: {}", total_value);
}
