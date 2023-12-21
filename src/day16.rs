use std::collections::HashSet;

use itertools::Itertools;
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    ThreadPoolBuilder,
};

use crate::utils::read_input;

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
    energize_map: &mut HashSet<(usize, usize)>,
    curr_idx: (usize, usize),
    dir: Direction,
) {
    if position_lookup.contains(&(curr_idx.0, curr_idx.1, dir)) {
        return;
    }
    energize_map.insert(curr_idx);
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

    let mut energize_map = HashSet::new();

    let mut position_lookup = HashSet::new();
    blast_beam(
        &mut position_lookup,
        &map,
        &mut energize_map,
        (0, 0),
        Direction::E,
    );
    println!("Day 16 Part 1: {}", energize_map.len());
}

fn calculate_energization(map: &Vec<Vec<char>>, idx: (usize, usize), direction: Direction) -> i32 {
    let mut energized_points = HashSet::new();

    let mut position_lookup = HashSet::new();
    blast_beam(
        &mut position_lookup,
        map,
        &mut energized_points,
        idx,
        direction,
    );
    let sum = energized_points.len();

    sum as i32
}

pub fn part2() {
    ThreadPoolBuilder::new()
        .stack_size(8 * 1024 * 1024)
        .build_global()
        .unwrap();

    let map = read_input("inputs/day16.txt")
        .into_iter()
        .map(|x| x.chars().clone().collect_vec())
        .collect_vec();

    let mut energization_params = vec![];

    for j in 0..map.len() {
        for i in 0..map[0].len() {
            if i == 0 {
                energization_params.push(((i, j), Direction::E));
            }

            if i == map[0].len() - 1 {
                energization_params.push(((i, j), Direction::W));
            }

            if j == 0 {
                energization_params.push(((i, j), Direction::S));
            }

            if j == map.len() - 1 {
                energization_params.push(((i, j), Direction::N));
            }
        }
    }

    let max_energized = energization_params
        .par_iter()
        .map(|x| calculate_energization(&map, x.0, x.1))
        .max()
        .unwrap();

    println!("Day 16 Part 2: {}", max_energized);
}
