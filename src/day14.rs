use std::collections::HashMap;

use crate::utils::read_input;

// fn print_map(map: &Vec<Vec<char>>) {
//     for row in map {
//         for col in row {
//             match col {
//                 '.' => print!("."),
//                 '#' => print!("#"),
//                 'O' => print!("O"),
//                 _ => unreachable!(),
//             }
//         }
//         println!();
//     }
// }

enum Direction {
    N,
    E,
    S,
    W,
}

fn roll(map: &mut Vec<Vec<char>>, dir: Direction) {
    match dir {
        Direction::N => {
            for j in 0..map[0].len() {
                for i in 0..map.len() {
                    let mut x = i;
                    let y = j;
                    if i > 0 && map[x][y] == 'O' {
                        while x > 0 {
                            if map[x - 1][y] == '.' {
                                map[x][y] = '.';
                                map[x - 1][y] = 'O';
                                x -= 1;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
        Direction::S => {
            for j in (0..map[0].len()).rev() {
                for i in (0..map.len()).rev() {
                    let mut x = i;
                    let y = j;
                    if i < map.len() - 1 && map[x][y] == 'O' {
                        while x < map.len() - 1 {
                            if map[x + 1][y] == '.' {
                                map[x][y] = '.';
                                map[x + 1][y] = 'O';
                                x += 1;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
        Direction::W => {
            for j in 0..map[0].len() {
                for i in 0..map.len() {
                    let x = i;
                    let mut y = j;
                    if j > 0 && map[x][y] == 'O' {
                        while y > 0 {
                            if map[x][y - 1] == '.' {
                                map[x][y] = '.';
                                map[x][y - 1] = 'O';
                                y -= 1;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
        Direction::E => {
            for j in (0..map[0].len()).rev() {
                for i in (0..map.len()).rev() {
                    let x = i;
                    let mut y = j;
                    if j < map[0].len() - 1 && map[x][y] == 'O' {
                        while y < map[0].len() - 1 {
                            if map[x][y + 1] == '.' {
                                map[x][y] = '.';
                                map[x][y + 1] = 'O';
                                y += 1;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn count_load(map: &Vec<Vec<char>>) -> i64 {
    let max_rows = map.len();
    let mut count = 0;
    for (idx, row) in map.iter().enumerate() {
        for col in row {
            if *col == 'O' {
                count += (max_rows - idx) as i64;
            }
        }
    }
    count
}

pub fn part1() {
    let mut map = read_input("inputs/day14.txt")
        .into_iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    roll(&mut map, Direction::N);
    println!("{}", count_load(&map));
}

fn roll_cycle(map: &mut Vec<Vec<char>>) {
    roll(map, Direction::N);
    roll(map, Direction::W);
    roll(map, Direction::S);
    roll(map, Direction::E);
}

fn deep_clone(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut clone_map = vec![];
    for row in map {
        clone_map.push(row.clone());
    }
    clone_map
}

pub fn part2() {
    let mut map = read_input("inputs/day14.txt")
        .into_iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut cycle_map: HashMap<Vec<Vec<char>>, Vec<i64>> = HashMap::new();
    let mut cycle_lookup: Vec<Vec<Vec<char>>> = vec![deep_clone(&map)];
    let mut idx = 0;

    let mut cycle_length = 0;
    let mut first_idx = 0;
    loop {
        idx += 1;
        roll_cycle(&mut map);
        cycle_lookup.push(deep_clone(&map));

        if cycle_map.contains_key(&map) {
            cycle_map.get_mut(&map).unwrap().push(idx);

            if cycle_map.get(&map).unwrap().len() == 2 {
                cycle_length = cycle_map.get(&map).unwrap()[1] - cycle_map.get(&map).unwrap()[0];
                first_idx = cycle_map.get(&map).unwrap()[0];
                break;
            }
        } else {
            cycle_map.insert(deep_clone(&map), vec![idx]);
        }
    }

    let cycle_rest = (1000000000 - first_idx) % cycle_length;
    let proper_cycle = &cycle_lookup[(cycle_rest + first_idx) as usize];
    println!("Day 14 Part 2: {}", count_load(proper_cycle));
}
