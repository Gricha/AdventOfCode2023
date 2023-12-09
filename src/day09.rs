use itertools::Itertools;

use crate::utils::read_input;

fn differentiate_vector(data: &Vec<i64>) -> Vec<i64> {
    let mut new_data = Vec::new();

    for i in 0..(data.len() - 1) {
        new_data.push(data[i + 1] - data[i]);
    }

    new_data
}

fn next_value(data: &Vec<i64>) -> i64 {
    if data.iter().all(|x| x == &0) {
        return 0;
    }

    let differentiated_vector = differentiate_vector(data);
    let next_val = data.iter().last().unwrap() + next_value(&differentiated_vector);
    next_val
}

fn prev_value(data: &Vec<i64>) -> i64 {
    if data.iter().all(|x| x == &0) {
        return 0;
    }

    let differentiated_vector = differentiate_vector(data);

    data[0] - prev_value(&differentiated_vector)
}

pub fn part1() {
    let sum = read_input("inputs/day9.txt")
        .into_iter()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect_vec()
        })
        .map(|data| next_value(&data))
        .sum::<i64>();
    println!("Day 9 Part 1: {}", sum);
}

pub fn part2() {
    let sum = read_input("inputs/day9.txt")
        .into_iter()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect_vec()
        })
        .map(|data| prev_value(&data))
        .sum::<i64>();
    println!("Day 9 Part 2: {}", sum);
}
