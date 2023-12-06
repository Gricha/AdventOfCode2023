use std::collections::HashSet;

use itertools::Itertools;
use num::traits::Pow;

use crate::utils::read_input;

pub fn part1() {
    println!(
        "Day 4 Part 1: {}",
        read_input("inputs/day4.txt")
            .into_iter()
            .map(|x| x.split_once(": ").unwrap().1.to_string())
            .map(|x| {
                x.split_once('|')
                    .map(|(a, b)| {
                        (
                            a.split_whitespace()
                                .map(|x| x.parse::<u32>().unwrap())
                                .collect_vec(),
                            b.split_whitespace()
                                .map(|x| x.parse::<u32>().unwrap())
                                .collect_vec(),
                        )
                    })
                    .unwrap()
            })
            .map(|(y, w)| {
                y.into_iter()
                    .collect::<HashSet<u32>>()
                    .intersection(&w.into_iter().collect::<HashSet<u32>>())
                    .count()
            })
            .map(|x| if x == 0 { 0 } else { 2.pow(x - 1) })
            .sum::<i32>()
    );
}

pub fn part2() {
    let card_data = read_input("inputs/day4.txt")
        .into_iter()
        .map(|x| x.split_once(": ").unwrap().1.to_string())
        .map(|x| {
            x.split_once('|')
                .map(|(a, b)| {
                    (
                        a.split_whitespace()
                            .map(|x| x.parse::<u32>().unwrap())
                            .collect_vec(),
                        b.split_whitespace()
                            .map(|x| x.parse::<u32>().unwrap())
                            .collect_vec(),
                    )
                })
                .unwrap()
        });
    // we start with each card existing, we will add copies
    let mut weights = (0..card_data.len()).map(|_| 1u64).collect_vec();
    for (idx, (y, w)) in card_data.enumerate() {
        let intersect = y
            .into_iter()
            .collect::<HashSet<u32>>()
            .intersection(&w.into_iter().collect::<HashSet<u32>>())
            .count();
        for i in 0..intersect {
            weights[idx + i + 1] += weights[idx];
        }
    }
    println!("Day 4 Part 2: {}", weights.into_iter().sum::<u64>());
}
