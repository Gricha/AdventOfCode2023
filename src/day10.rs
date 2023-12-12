use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::read_input;

enum Direction {
    N,
    S,
    W,
    E,
}

fn does_it_loop(
    map: &[Vec<char>],
    start_pos: (u32, u32),
    start_index: (u32, u32),
    direction: Direction,
) -> Option<Vec<(u32, u32)>> {
    // starting from start index walks around the map and finds out if it ends up in a loop
    let max_row = map.len() as u32;
    let max_col = map[0].len() as u32;

    let mut path = vec![start_index];

    let mut curr = start_index;
    let mut dir = direction;
    loop {
        if curr == start_pos {
            return Some(path);
        }

        match map[curr.1 as usize][curr.0 as usize] {
            '.' => return None,
            '|' => match dir {
                Direction::N => {
                    if curr.1 == 0 {
                        return None;
                    }

                    curr.1 -= 1;
                    path.push(curr);
                }
                Direction::S => {
                    if curr.1 == max_row - 1 {
                        return None;
                    }

                    curr.1 += 1;
                    path.push(curr);
                }
                // came from wrong direction, hit the wall
                Direction::E | Direction::W => return None,
            },
            '-' => {
                match dir {
                    Direction::E => {
                        if curr.0 == max_col - 1 {
                            return None;
                        }

                        curr.0 += 1;
                        path.push(curr);
                    }
                    Direction::W => {
                        if curr.0 == 0 {
                            return None;
                        }

                        curr.0 -= 1;
                        path.push(curr);
                    }
                    // came from wrong direction, hit the wall
                    Direction::N | Direction::S => return None,
                }
            }

            // bends
            'L' => match dir {
                Direction::S => {
                    if curr.0 == max_col - 1 {
                        return None;
                    }

                    curr.0 += 1;
                    dir = Direction::E;
                    path.push(curr);
                }
                Direction::W => {
                    if curr.1 == 0 {
                        return None;
                    }

                    curr.1 -= 1;
                    dir = Direction::N;
                    path.push(curr);
                }
                Direction::N | Direction::E => return None,
            },

            'J' => match dir {
                Direction::S => {
                    if curr.0 == 0 {
                        return None;
                    }

                    curr.0 -= 1;
                    dir = Direction::W;
                    path.push(curr);
                }
                Direction::E => {
                    if curr.1 == 0 {
                        return None;
                    }

                    curr.1 -= 1;
                    dir = Direction::N;
                    path.push(curr);
                }
                Direction::N | Direction::W => return None,
            },

            '7' => match dir {
                Direction::N => {
                    if curr.0 == 0 {
                        return None;
                    }

                    curr.0 -= 1;
                    dir = Direction::W;
                    path.push(curr);
                }
                Direction::E => {
                    if curr.1 == max_row - 1 {
                        return None;
                    }

                    curr.1 += 1;
                    dir = Direction::S;
                    path.push(curr);
                }
                Direction::S | Direction::W => return None,
            },

            'F' => match dir {
                Direction::N => {
                    if curr.0 == max_col - 1 {
                        return None;
                    }

                    curr.0 += 1;
                    dir = Direction::E;
                    path.push(curr);
                }
                Direction::W => {
                    if curr.1 == max_row - 1 {
                        return None;
                    }

                    curr.1 += 1;
                    dir = Direction::S;
                    path.push(curr);
                }
                Direction::S | Direction::E => return None,
            },

            _ => unreachable!(),
        }
    }
}

fn find_loop(map: &[Vec<char>]) -> Vec<(u32, u32)> {
    let mut start_pos = (0, 0);
    for (y, row) in map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == 'S' {
                start_pos = (x as u32, y as u32);
            }
        }
    }
    let mut potential_cycles = vec![];
    let max_row = map.len() as u32;
    let max_col = map[0].len() as u32;

    if start_pos.0 > 0 {
        potential_cycles.push(does_it_loop(
            map,
            start_pos,
            (start_pos.0 - 1, start_pos.1),
            Direction::W,
        ));
    }
    if start_pos.0 < max_col - 1 {
        potential_cycles.push(does_it_loop(
            map,
            start_pos,
            (start_pos.0 + 1, start_pos.1),
            Direction::E,
        ));
    }
    if start_pos.1 > 0 {
        potential_cycles.push(does_it_loop(
            map,
            start_pos,
            (start_pos.0, start_pos.1 - 1),
            Direction::N,
        ));
    }
    if start_pos.1 < max_row - 1 {
        potential_cycles.push(does_it_loop(
            map,
            start_pos,
            (start_pos.0, start_pos.1 + 1),
            Direction::S,
        ));
    }

    potential_cycles.into_iter().flatten().next().unwrap()
}

pub fn part1() {
    let map = read_input("inputs/day10.txt")
        .into_iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect_vec();

    let any_path_that_loops = find_loop(&map);

    println!(
        "Day 10 part 1: {}",
        (any_path_that_loops.len() / 2) + any_path_that_loops.len() % 2
    );
}

fn direction_from_points(prev: &(u32, u32), next: &(u32, u32)) -> Direction {
    #[allow(clippy::comparison_chain)]
    if prev.0 == next.0 {
        if prev.1 > next.1 {
            Direction::N
        } else {
            Direction::S
        }
    } else if prev.0 < next.0 {
        Direction::E
    } else {
        Direction::W
    }
}

fn flood_fill_point(map: &mut Vec<Vec<char>>, point: (u32, u32), sign: char) {
    let max_row = map.len() as u32;
    let max_col = map[0].len() as u32;

    if map[point.1 as usize][point.0 as usize] == sign {
        return;
    }

    if map[point.1 as usize][point.0 as usize] == '.' {
        map[point.1 as usize][point.0 as usize] = sign;
        // directionally flood each side
        if point.1 > 0 {
            flood_fill_point(map, (point.0, point.1 - 1), sign);
        }
        if point.1 < max_row - 1 {
            flood_fill_point(map, (point.0, point.1 + 1), sign);
        }

        if point.0 > 0 {
            flood_fill_point(map, (point.0 - 1, point.1), sign);
        }

        if point.0 < max_col - 1 {
            flood_fill_point(map, (point.0 + 1, point.1), sign);
        }
    }
}

fn flood_fill(map: &mut Vec<Vec<char>>, point: (u32, u32), prev_dir: Direction) {
    let max_row = map.len() as u32;
    let max_col = map[0].len() as u32;

    match map[point.1 as usize][point.0 as usize] {
        'S' => {}

        '-' => match prev_dir {
            Direction::W => {
                if point.1 > 0 {
                    flood_fill_point(map, (point.0, point.1 - 1), 'I');
                }
            }
            Direction::E => {
                if point.1 < max_row - 1 {
                    flood_fill_point(map, (point.0, point.1 + 1), 'I');
                }
            }
            _ => unreachable!(),
        },
        '|' => match prev_dir {
            Direction::S => {
                if point.0 > 0 {
                    flood_fill_point(map, (point.0 - 1, point.1), 'I');
                }
            }
            Direction::N => {
                if point.0 < max_col - 1 {
                    flood_fill_point(map, (point.0 + 1, point.1), 'I');
                }
            }
            _ => unreachable!(),
        },
        'L' => match prev_dir {
            Direction::S => {
                if point.0 > 0 {
                    flood_fill_point(map, (point.0 - 1, point.1), 'I');
                }
                if point.1 < max_row - 1 {
                    flood_fill_point(map, (point.0, point.1 + 1), 'I');
                }
            }
            Direction::W => {
                // nothing
            }
            _ => unreachable!(),
        },
        'F' => match prev_dir {
            Direction::N => {
                // no coloring
            }
            Direction::W => {
                // coloring both top and left
                if point.0 > 0 {
                    flood_fill_point(map, (point.0 - 1, point.1), 'I');
                }
                if point.1 > 0 {
                    flood_fill_point(map, (point.0, point.1 - 1), 'I');
                }
            }
            _ => unreachable!(),
        },
        '7' => match prev_dir {
            Direction::N => {
                if point.0 < max_col - 1 {
                    flood_fill_point(map, (point.0 + 1, point.1), 'I');
                }
                if point.1 > 0 {
                    flood_fill_point(map, (point.0, point.1 - 1), 'I');
                }
            }
            Direction::E => {
                // nothing
            }
            _ => unreachable!(),
        },
        'J' => match prev_dir {
            Direction::E => {
                if point.0 < max_col - 1 {
                    flood_fill_point(map, (point.0 + 1, point.1), 'I');
                }
                if point.1 < max_row - 1 {
                    flood_fill_point(map, (point.0, point.1 + 1), 'I');
                }
            }
            Direction::S => {
                // nothing
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

pub fn part2() {
    let map = read_input("inputs/day10.txt")
        .into_iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect_vec();

    let path = find_loop(&map);
    let points_as_set = path.clone().into_iter().collect::<HashSet<(u32, u32)>>();
    let redrawn_map = map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, col)| {
                    if !points_as_set.contains(&(x as u32, y as u32)) {
                        '.'
                    } else {
                        *col
                    }
                })
                .collect_vec()
        })
        .collect_vec();
    // print_map(&redrawn_map);

    let mut map = redrawn_map;

    let steps = path.len();

    // double up
    let mut new_path = path.clone();
    for p in path.into_iter() {
        new_path.push(p);
    }

    let mut idx = 1;
    while idx <= steps {
        let prev = new_path[idx - 1];
        let curr = new_path[idx];

        let prev_dir = direction_from_points(&prev, &curr);

        flood_fill(&mut map, curr, prev_dir);

        idx += 1;
    }

    let mut count_i = 0;
    let mut count_o = 0;
    let mut is_i_inner = false;
    for (i, row) in map.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col == &'I' {
                count_i += 1;
            } else if col == &'.' {
                count_o += 1;
                if i == 0 || i == map.len() - 1 || j == 0 || j == row.len() - 1 {
                    is_i_inner = true;
                }
            }
        }
    }

    let mut inner_size = count_i;
    if !is_i_inner {
        inner_size = count_o;
    }

    println!("Day 10 part 2: {}", inner_size)
}
