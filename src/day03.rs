use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::read_input;

fn number_at_index(data: &[Vec<char>], row: usize, col: usize) -> Option<u32> {
    if !data[row][col].is_ascii_digit() {
        return None;
    }

    // find left
    let mut left = col;
    loop {
        if left == 0 {
            break;
        }
        if !data[row][left - 1].is_ascii_digit() {
            break;
        }
        left -= 1;
    }

    // now get all digits from left to right
    let mut digits = Vec::new();
    for i in left..data[row].len() {
        if !data[row][i].is_ascii_digit() {
            break;
        }
        digits.push(data[row][i]);
    }

    Some(
        digits
            .iter()
            .rev()
            .enumerate()
            .map(|(i, x)| x.to_digit(10).unwrap() * 10u32.pow(i as u32))
            .sum(),
    )
}

pub fn get_all_parts_around_index(data: &Vec<Vec<char>>, row: usize, col: usize) -> HashSet<u32> {
    let mut parts = HashSet::new();
    let max_row = data.len();
    let max_col = data[0].len();

    if col > 0 {
        if let Some(num) = number_at_index(data, row, col - 1) {
            parts.insert(num);
        }
    }

    if col < max_col - 1 {
        if let Some(num) = number_at_index(data, row, col + 1) {
            parts.insert(num);
        }
    }

    if row > 0 {
        if let Some(num) = number_at_index(data, row - 1, col) {
            parts.insert(num);
        }
    }

    if row < max_row - 1 {
        if let Some(num) = number_at_index(data, row + 1, col) {
            parts.insert(num);
        }
    }

    if row > 0 && col > 0 {
        if let Some(num) = number_at_index(data, row - 1, col - 1) {
            parts.insert(num);
        }
    }

    if row > 0 && col < max_col - 1 {
        if let Some(num) = number_at_index(data, row - 1, col + 1) {
            parts.insert(num);
        }
    }

    if row < max_row - 1 && col > 0 {
        if let Some(num) = number_at_index(data, row + 1, col - 1) {
            parts.insert(num);
        }
    }

    if row < max_row - 1 && col < max_col - 1 {
        if let Some(num) = number_at_index(data, row + 1, col + 1) {
            parts.insert(num);
        }
    }

    parts
}

pub fn part1() {
    let lines = read_input("inputs/day3.txt")
        .into_iter()
        .map(|x| x.chars().collect_vec())
        .collect_vec();
    let rows = lines.len();
    let cols = lines[0].len();

    let mut all_parts = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            if lines[row][col].is_ascii_digit() || lines[row][col] == '.' {
                continue;
            }
            let parts = get_all_parts_around_index(&lines, row, col);
            all_parts.extend(parts);
        }
    }

    println!("Day 3 Part 1: {}", all_parts.into_iter().sum::<u32>());
}

pub fn part2() {
    let lines = read_input("inputs/day3.txt")
        .into_iter()
        .map(|x| x.chars().collect_vec())
        .collect_vec();
    let rows = lines.len();
    let cols = lines[0].len();

    let mut gear_ratios = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            if lines[row][col] == '*' {
                let parts = get_all_parts_around_index(&lines, row, col);
                if parts.len() == 2 {
                    gear_ratios.push(parts.into_iter().product::<u32>())
                }
            }
        }
    }

    println!("Day 3 Part 2: {}", gear_ratios.into_iter().sum::<u32>());
}
