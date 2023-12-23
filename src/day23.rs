use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::read_input;

fn dfs(
    map: &[Vec<char>],
    stepped: &mut Vec<Vec<bool>>,
    max_steps_to_dest: &mut Vec<usize>,
    pos: (usize, usize),
    current_steps: usize,
) {
    if pos.1 == map.len() - 1 {
        max_steps_to_dest.push(current_steps);
        return;
    }

    if pos.0 > 0
        && (map[pos.1][pos.0 - 1] == '.' || map[pos.1][pos.0 - 1] == '<')
        && !stepped[pos.1][pos.0 - 1]
    {
        stepped[pos.1][pos.0 - 1] = true;
        dfs(
            map,
            stepped,
            max_steps_to_dest,
            (pos.0 - 1, pos.1),
            current_steps + 1,
        );
        stepped[pos.1][pos.0 - 1] = false;
    }

    if pos.0 < map[0].len() - 1
        && (map[pos.1][pos.0 + 1] == '.' || map[pos.1][pos.0 + 1] == '>')
        && !stepped[pos.1][pos.0 + 1]
    {
        stepped[pos.1][pos.0 + 1] = true;
        dfs(
            map,
            stepped,
            max_steps_to_dest,
            (pos.0 + 1, pos.1),
            current_steps + 1,
        );
        stepped[pos.1][pos.0 + 1] = false;
    }

    if pos.1 > 0
        && (map[pos.1 - 1][pos.0] == '.' || map[pos.1 - 1][pos.0] == '^')
        && !stepped[pos.1 - 1][pos.0]
    {
        stepped[pos.1 - 1][pos.0] = true;
        dfs(
            map,
            stepped,
            max_steps_to_dest,
            (pos.0, pos.1 - 1),
            current_steps + 1,
        );
        stepped[pos.1 - 1][pos.0] = false;
    }

    if pos.1 < map.len() - 1
        && (map[pos.1 + 1][pos.0] == '.' || map[pos.1 + 1][pos.0] == 'v')
        && !stepped[pos.1 + 1][pos.0]
    {
        stepped[pos.1 + 1][pos.0] = true;
        dfs(
            map,
            stepped,
            max_steps_to_dest,
            (pos.0, pos.1 + 1),
            current_steps + 1,
        );
        stepped[pos.1 + 1][pos.0] = false;
    }
}

pub fn part1() {
    let map = read_input("inputs/day23.txt")
        .into_iter()
        .map(|x| x.chars().collect_vec())
        .collect_vec();
    let mut stepped = vec![vec![false; map[0].len()]; map.len()];
    let mut max_steps_to_dest = vec![];

    dfs(&map, &mut stepped, &mut max_steps_to_dest, (1, 0), 0);

    println!("Day 23 Part 1: {}", max_steps_to_dest.iter().max().unwrap());
}

fn dfs_with_adjacency(
    map: &HashMap<(usize, usize), Vec<(usize, usize, i64)>>,
    stepped: &mut HashSet<(usize, usize)>,
    max_steps_to_dest: &mut Vec<usize>,
    pos: (usize, usize),
    current_steps: usize,
    final_pos: (usize, usize),
) {
    // println!(
    //     "Visited {} {} {} {}",
    //     pos.0,
    //     pos.1,
    //     current_steps,
    //     map.len() - 1
    // );
    if pos == final_pos {
        max_steps_to_dest.push(current_steps);
        return;
    }

    for (x, y, cost) in map.get(&pos).unwrap() {
        if !stepped.contains(&(*x, *y)) {
            stepped.insert((*x, *y));
            dfs_with_adjacency(
                map,
                stepped,
                max_steps_to_dest,
                (*x, *y),
                current_steps + *cost as usize,
                final_pos,
            );
            stepped.remove(&(*x, *y));
        }
    }
}

fn build_adjacency_map(map: &Vec<Vec<char>>) -> HashMap<(usize, usize), Vec<(usize, usize, i64)>> {
    let mut adjacency = HashMap::new();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '.' {
                let mut adjacent_nodes = vec![];

                if x > 0 && map[y][x - 1] == '.' {
                    adjacent_nodes.push((x - 1, y, 1));
                }

                if x < map[0].len() - 1 && map[y][x + 1] == '.' {
                    adjacent_nodes.push((x + 1, y, 1));
                }

                if y > 0 && map[y - 1][x] == '.' {
                    adjacent_nodes.push((x, y - 1, 1));
                }

                if y < map.len() - 1 && map[y + 1][x] == '.' {
                    adjacent_nodes.push((x, y + 1, 1));
                }

                adjacency.insert((x, y), adjacent_nodes);
            }
        }
    }

    adjacency
}

fn reduce_adjacency_map(map: &mut HashMap<(usize, usize), Vec<(usize, usize, i64)>>) {
    let mut reduced = true;

    while reduced {
        reduced = false;

        let keys = map.keys().cloned().collect_vec();

        for pos in keys {
            let adjacent_nodes = map.get(&pos).unwrap().clone();
            if adjacent_nodes.len() == 2 {
                let (x1, y1, cost1) = adjacent_nodes[0];
                let (x2, y2, cost2) = adjacent_nodes[1];

                let mut new_adjacent_nodes_1 = map.get(&(x1, y1)).unwrap().clone();
                let mut new_adjacent_nodes_2 = map.get(&(x2, y2)).unwrap().clone();
                new_adjacent_nodes_1.push((x2, y2, cost1 + cost2));
                new_adjacent_nodes_2.push((x1, y1, cost1 + cost2));

                map.remove(&pos);

                map.insert(
                    (x1, y1),
                    new_adjacent_nodes_1
                        .into_iter()
                        .filter(|(x, y, _)| *x != pos.0 || *y != pos.1)
                        .collect_vec(),
                );

                map.insert(
                    (x2, y2),
                    new_adjacent_nodes_2
                        .into_iter()
                        .filter(|(x, y, _)| *x != pos.0 || *y != pos.1)
                        .collect_vec(),
                );

                reduced = true;
                break;
            }
        }
    }
}

pub fn part2() {
    let map = read_input("inputs/day23.txt")
        .into_iter()
        .map(|x| {
            x.chars()
                .map(|x| match x {
                    '<' | '>' | '^' | 'v' => '.',
                    _ => x,
                })
                .collect_vec()
        })
        .collect_vec();
    let final_pos = (map[0].len() - 2, map.len() - 1);

    let mut adjacency = build_adjacency_map(&map);
    reduce_adjacency_map(&mut adjacency);

    let mut stepped = HashSet::new();
    let mut max_steps_to_dest = vec![];
    // dbg!(&adjacency.iter().filter(|(_, v)| v.len() == 1).collect_vec());
    dfs_with_adjacency(
        &adjacency,
        &mut stepped,
        &mut max_steps_to_dest,
        (1, 0),
        0,
        final_pos,
    );

    println!("Day 23 Part 2: {}", max_steps_to_dest.iter().max().unwrap());
}
