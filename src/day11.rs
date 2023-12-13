use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::read_input;

// fn print_map(map: &Vec<Vec<i64>>) {
//     for row in map {
//         for col in row {
//             print!("{}", *col);
//         }
//         println!();
//     }
// }

pub fn part1() {
    let mut initial_map = read_input("inputs/day11.txt")
        .into_iter()
        .map(|x| {
            x.chars()
                .map(|x| if x == '.' { 0 } else { 1 })
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let mut empty_rows = vec![];
    let mut empty_columns = vec![];

    let max_row = initial_map.len();
    let max_col = initial_map[0].len();

    for i in 0..max_row {
        if initial_map[i].iter().sum::<i64>() == 0 {
            empty_rows.push(i);
        }
    }

    for i in 0..max_col {
        if initial_map.iter().map(|x| x[i]).sum::<i64>() == 0 {
            empty_columns.push(i);
        }
    }

    // dbg!(empty_rows, empty_columns);
    let new_row = vec![0; max_col];
    for i in empty_rows.into_iter().rev() {
        initial_map.insert(i, new_row.clone());
    }

    for i in empty_columns.into_iter().rev() {
        for row in initial_map.iter_mut() {
            row.insert(i, 0);
        }
    }

    let max_row = initial_map.len();
    let max_col = initial_map[0].len();

    let mut galaxy_locations = vec![];
    for i in 0..max_row {
        for j in 0..max_col {
            if initial_map[i][j] == 1 {
                galaxy_locations.push((i as i64, j as i64));
            }
        }
    }

    let pairs = galaxy_locations
        .clone()
        .into_iter()
        .cartesian_product(galaxy_locations)
        .filter(|x| x.0 != x.1)
        .fold(
            HashSet::<((i64, i64), (i64, i64))>::new(),
            |mut acc, value| {
                if acc.contains(&(value.1, value.0)) {
                    acc
                } else {
                    acc.insert(value);
                    acc
                }
            },
        );

    let distances = pairs
        .iter()
        .map(|((a1, a2), (b1, b2))| ((a1 - b1).abs() + (a2 - b2).abs()))
        .sum::<i64>();

    println!("Day 11 Part 1: {}", distances);
}

pub fn part2() {
    let initial_map = read_input("inputs/day11.txt")
        .into_iter()
        .map(|x| {
            x.chars()
                .map(|x| if x == '.' { 0 } else { 1 })
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let mut empty_rows = vec![];
    let mut empty_columns = vec![];

    let max_row = initial_map.len();
    let max_col = initial_map[0].len();

    for i in 0..max_row {
        if initial_map[i].iter().sum::<i64>() == 0 {
            empty_rows.push(i);
        }
    }

    for i in 0..max_col {
        if initial_map.iter().map(|x| x[i]).sum::<i64>() == 0 {
            empty_columns.push(i);
        }
    }

    let mut debug_map = initial_map.clone();
    for j in 0..max_row {
        for i in 0..max_col {
            if empty_columns.contains(&j) || empty_rows.contains(&i) {
                debug_map[i][j] = 2;
            }
        }
    }

    let mut distance_map = vec![vec![0; max_col]; max_row];
    for j in 0..max_row {
        for i in 0..max_col {
            if empty_columns.contains(&j) || empty_rows.contains(&i) {
                distance_map[i][j] = 1000000;
            } else {
                distance_map[i][j] = 1;
            }
        }
    }

    let mut galaxy_locations = vec![];
    for i in 0..max_row {
        for j in 0..max_col {
            if initial_map[i][j] == 1 {
                galaxy_locations.push((i as i64, j as i64));
            }
        }
    }

    let pairs = galaxy_locations
        .clone()
        .into_iter()
        .cartesian_product(galaxy_locations)
        .filter(|x| x.0 != x.1)
        .fold(
            HashSet::<((i64, i64), (i64, i64))>::new(),
            |mut acc, value| {
                if acc.contains(&(value.1, value.0)) {
                    acc
                } else {
                    acc.insert(value);
                    acc
                }
            },
        );

    let distances = pairs
        .iter()
        .map(|((a1, a2), (b1, b2))| {
            let mut distance = 0;
            let starting_point_x = *std::cmp::min(a1, b1);
            let endpoint_point_x = *std::cmp::max(a1, b1);
            let starting_point_y = *std::cmp::min(a2, b2);
            let endpoint_point_y = *std::cmp::max(a2, b2);

            for i in starting_point_x..endpoint_point_x {
                distance += distance_map[i as usize][*a2 as usize];
            }
            for j in starting_point_y..endpoint_point_y {
                distance += distance_map[*b1 as usize][j as usize];
            }
            distance
        })
        .sum::<i64>();

    println!("Day 11 Part 2: {}", distances);
}
