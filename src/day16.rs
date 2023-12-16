use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::read_input;

fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

fn print_energize_map(map: &Vec<Vec<i32>>) {
    for row in map {
        for col in row {
            match col {
                0 => print!("."),
                1 => print!("#"),
                _ => unreachable!(),
            }
        }
        println!();
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    N,
    E,
    W,
    S,
}

fn blast_beam(
    position_lookup: &mut HashSet<(usize, usize, Direction)>,
    map: &Vec<Vec<char>>,
    energize_map: &mut Vec<Vec<i32>>,
    curr_idx: (usize, usize),
    dir: Direction,
) {
    if position_lookup.contains(&(curr_idx.0, curr_idx.1, dir)) {
        return;
    }
    energize_map[curr_idx.1][curr_idx.0] = 1;
    position_lookup.insert((curr_idx.0, curr_idx.1, dir));

    match &map[curr_idx.1][curr_idx.0] {
        '.' => match dir {
            Direction::N => {
                if curr_idx.1 > 0 {
                    blast_beam(
                        position_lookup,
                        map,
                        energize_map,
                        (curr_idx.0, curr_idx.1 - 1),
                        dir,
                    );
                }
            }
            Direction::E => {
                if curr_idx.0 < map[0].len() - 1 {
                    blast_beam(
                        position_lookup,
                        map,
                        energize_map,
                        (curr_idx.0 + 1, curr_idx.1),
                        dir,
                    );
                }
            }
            Direction::W => {
                if curr_idx.0 > 0 {
                    blast_beam(
                        position_lookup,
                        map,
                        energize_map,
                        (curr_idx.0 - 1, curr_idx.1),
                        dir,
                    );
                }
            }
            Direction::S => {
                if curr_idx.1 < map.len() - 1 {
                    blast_beam(
                        position_lookup,
                        map,
                        energize_map,
                        (curr_idx.0, curr_idx.1 + 1),
                        dir,
                    );
                }
            }
        },
        '-' => match dir {
            Direction::E => {
                if curr_idx.0 < map[0].len() - 1 {
                    blast_beam(
                        position_lookup,
                        map,
                        energize_map,
                        (curr_idx.0 + 1, curr_idx.1),
                        dir,
                    );
                }
            }
            Direction::W => {
                if curr_idx.0 > 0 {
                    blast_beam(
                        position_lookup,
                        map,
                        energize_map,
                        (curr_idx.0 - 1, curr_idx.1),
                        dir,
                    );
                }
            }
            Direction::N | Direction::S => {
                if curr_idx.0 > 0 {
                    blast_beam(
                        position_lookup,
                        map,
                        energize_map,
                        (curr_idx.0 - 1, curr_idx.1),
                        Direction::W,
                    );
                }

                if curr_idx.0 < map[0].len() - 1 {
                    blast_beam(
                        position_lookup,
                        map,
                        energize_map,
                        (curr_idx.0 + 1, curr_idx.1),
                        Direction::E,
                    );
                }
            }
        },
        '|' => match dir {
            Direction::N => {
                if curr_idx.1 > 0 {
                    blast_beam(
                        position_lookup,
                        map,
                        energize_map,
                        (curr_idx.0, curr_idx.1 - 1),
                        dir,
                    );
                }
            }
            Direction::S => {
                if curr_idx.1 < map.len() - 1 {
                    blast_beam(
                        position_lookup,
                        map,
                        energize_map,
                        (curr_idx.0, curr_idx.1 + 1),
                        dir,
                    );
                }
            }
            Direction::E | Direction::W => {
                if curr_idx.1 > 0 {
                    blast_beam(
                        position_lookup,
                        map,
                        energize_map,
                        (curr_idx.0, curr_idx.1 - 1),
                        Direction::N,
                    );
                }

                if curr_idx.1 < map.len() - 1 {
                    blast_beam(
                        position_lookup,
                        map,
                        energize_map,
                        (curr_idx.0, curr_idx.1 + 1),
                        Direction::S,
                    );
                }
            }
        },
        '/' => match dir {
            Direction::N => {
                if curr_idx.0 < map[0].len() - 1 {
                    blast_beam(
                        position_lookup,
                        map,
                        energize_map,
                        (curr_idx.0 + 1, curr_idx.1),
                        Direction::E,
                    );
                }
            }
            Direction::S => {
                if curr_idx.0 > 0 {
                    blast_beam(
                        position_lookup,
                        map,
                        energize_map,
                        (curr_idx.0 - 1, curr_idx.1),
                        Direction::W,
                    );
                }
            }
            Direction::W => {
                if curr_idx.1 < map.len() - 1 {
                    blast_beam(
                        position_lookup,
                        map,
                        energize_map,
                        (curr_idx.0, curr_idx.1 + 1),
                        Direction::S,
                    );
                }
            }
            Direction::E => {
                if curr_idx.1 > 0 {
                    blast_beam(
                        position_lookup,
                        map,
                        energize_map,
                        (curr_idx.0, curr_idx.1 - 1),
                        Direction::N,
                    );
                }
            }
        },
        '\\' => match dir {
            Direction::N => {
                if curr_idx.0 > 0 {
                    blast_beam(
                        position_lookup,
                        map,
                        energize_map,
                        (curr_idx.0 - 1, curr_idx.1),
                        Direction::W,
                    );
                }
            }
            Direction::S => {
                if curr_idx.0 < map[0].len() - 1 {
                    blast_beam(
                        position_lookup,
                        map,
                        energize_map,
                        (curr_idx.0 + 1, curr_idx.1),
                        Direction::E,
                    );
                }
            }
            Direction::E => {
                if curr_idx.1 < map.len() - 1 {
                    blast_beam(
                        position_lookup,
                        map,
                        energize_map,
                        (curr_idx.0, curr_idx.1 + 1),
                        Direction::S,
                    );
                }
            }
            Direction::W => {
                if curr_idx.1 > 0 {
                    blast_beam(
                        position_lookup,
                        map,
                        energize_map,
                        (curr_idx.0, curr_idx.1 - 1),
                        Direction::N,
                    );
                }
            }
        },
        _ => unreachable!(),
    }
}

pub fn part1() {
    let map = read_input("inputs/day16.txt")
        .into_iter()
        .map(|x| x.chars().clone().collect_vec())
        .collect_vec();

    let mut energize_map = vec![];
    for _ in 0..map.len() {
        let energize_row = vec![0; map.len()];
        energize_map.push(energize_row);
    }

    let mut position_lookup = HashSet::new();
    blast_beam(
        &mut position_lookup,
        &map,
        &mut energize_map,
        (0, 0),
        Direction::E,
    );
    println!(
        "Day 16 Part 1: {}",
        energize_map.iter().flatten().sum::<i32>()
    );
}

fn calculate_energization(map: &Vec<Vec<char>>, idx: (usize, usize), direction: Direction) -> i32 {
    let mut energize_map = vec![];
    for _ in 0..map.len() {
        let energize_row = vec![0; map.len()];
        energize_map.push(energize_row);
    }

    let mut position_lookup = HashSet::new();
    blast_beam(&mut position_lookup, map, &mut energize_map, idx, direction);
    let sum = energize_map.iter().flatten().sum::<i32>();

    sum
}

pub fn part2() {
    let map = read_input("inputs/day16.txt")
        .into_iter()
        .map(|x| x.chars().clone().collect_vec())
        .collect_vec();

    let mut max_energized = 0;
    for j in 0..map.len() {
        for i in 0..map[0].len() {
            if i == 0 {
                max_energized = std::cmp::max(
                    max_energized,
                    calculate_energization(&map, (i, j), Direction::E),
                );
            }

            if i == map[0].len() - 1 {
                max_energized = std::cmp::max(
                    max_energized,
                    calculate_energization(&map, (i, j), Direction::W),
                );
            }

            if j == 0 {
                max_energized = std::cmp::max(
                    max_energized,
                    calculate_energization(&map, (i, j), Direction::S),
                );
            }

            if j == map.len() - 1 {
                max_energized = std::cmp::max(
                    max_energized,
                    calculate_energization(&map, (i, j), Direction::N),
                );
            }
        }
    }

    println!("Day 16 Part 2: {}", max_energized);
}
