use itertools::Itertools;

use crate::utils::read_input;

pub fn part1() {
    let input = read_input("inputs/day1.txt");

    let sum: u32 = input
        .iter()
        .map(|x| {
            (x.chars().find(|x| x.is_ascii_digit()))
                .unwrap()
                .to_digit(10)
                .unwrap()
                * 10
                + (x.chars()
                    .rev()
                    .find(|x| x.is_ascii_digit())
                    .unwrap()
                    .to_digit(10)
                    .unwrap())
        })
        .sum();

    println!("Day 1 Part 1: {}", sum);
}

pub fn part2() {
    let input = read_input("inputs/day1.txt");

    let input = input
        .into_iter()
        .map(|line| {
            line.replace("zero", "ze0ro")
                .replace("one", "o1ne")
                .replace("two", "t2wo")
                .replace("three", "thr3ee")
                .replace("four", "fo4ur")
                .replace("five", "fi5ve")
                .replace("six", "s6ix")
                .replace("seven", "se7ven")
                .replace("eight", "eig8ht")
                .replace("nine", "ni9ne")
        })
        .collect_vec();

    let sum: u32 = input
        .iter()
        .map(|x| {
            (x.chars().find(|x| x.is_ascii_digit()))
                .unwrap()
                .to_digit(10)
                .unwrap()
                * 10
                + (x.chars()
                    .rev()
                    .find(|x| x.is_ascii_digit())
                    .unwrap()
                    .to_digit(10)
                    .unwrap())
        })
        .sum();

    println!("Day 1 Part 1: {}", sum);
}
