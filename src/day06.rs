use itertools::Itertools;

use crate::utils::read_input;

pub fn part1() {
    let lines = read_input("inputs/day6.txt")
        .into_iter()
        .map(|x| {
            x.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut data = Vec::new();

    for i in 0..lines[0].len() {
        data.push((lines[0][i], lines[1][i]));
    }

    println!(
        "Day 6 Part 1: {}",
        data.into_iter()
            .map(|(time, distance)| {
                let mut score = 0;
                for charge in 0..time {
                    let final_distance = (time - charge) * charge;
                    if final_distance > distance {
                        score += 1;
                    }
                }

                score
            })
            .product::<u64>()
    );
}

pub fn part2() {
    let lines = read_input("inputs/day6.txt")
        .into_iter()
        .map(|x| {
            let val = x.split_once(':').unwrap().1.split_whitespace().join("");
            val.parse::<u64>().unwrap()
        })
        .collect_vec();

    let time = lines[0];
    let distance = lines[1];

    let mut score = 0;
    for charge in 0..time {
        let final_distance = (time - charge) * charge;
        if final_distance > distance {
            score += 1;
        }
    }

    println!("Day 6 Part 1: {}", score);
}
