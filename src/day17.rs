use itertools::Itertools;

use crate::{
    dijkstra::{dijkstra, Neighbor},
    utils::read_input,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    direction: Direction,
    // cant take more than 3 steps in that direction for it to be considered a valid neighbor
    steps: u8,
    idx: (usize, usize),
}

fn get_neighbors(map: &Vec<Vec<u32>>, node: &Node, min_length: u8, max_length: u8) -> Vec<Node> {
    let mut nodes = vec![];

    let (x, y) = node.idx;

    match node.direction {
        Direction::N => {
            if node.steps < max_length && y > 0 {
                nodes.push(Node {
                    direction: Direction::N,
                    steps: node.steps + 1,
                    idx: (x, y - 1),
                })
            }
            if node.steps >= min_length && x > 0 {
                nodes.push(Node {
                    direction: Direction::W,
                    steps: 1,
                    idx: (x - 1, y),
                })
            }
            if node.steps >= min_length && x < map[0].len() - 1 {
                nodes.push(Node {
                    direction: Direction::E,
                    steps: 1,
                    idx: (x + 1, y),
                })
            }
        }
        Direction::S => {
            if node.steps < max_length && y < map.len() - 1 {
                nodes.push(Node {
                    direction: Direction::S,
                    steps: node.steps + 1,
                    idx: (x, y + 1),
                })
            }
            if node.steps >= min_length && x > 0 {
                nodes.push(Node {
                    direction: Direction::W,
                    steps: 1,
                    idx: (x - 1, y),
                })
            }
            if node.steps >= min_length && x < map[0].len() - 1 {
                nodes.push(Node {
                    direction: Direction::E,
                    steps: 1,
                    idx: (x + 1, y),
                })
            }
        }

        Direction::W => {
            if node.steps < max_length && x > 0 {
                nodes.push(Node {
                    direction: Direction::W,
                    steps: node.steps + 1,
                    idx: (x - 1, y),
                })
            }
            if node.steps >= min_length && y > 0 {
                nodes.push(Node {
                    direction: Direction::N,
                    steps: 1,
                    idx: (x, y - 1),
                })
            }
            if node.steps >= min_length && y < map.len() - 1 {
                nodes.push(Node {
                    direction: Direction::S,
                    steps: 1,
                    idx: (x, y + 1),
                })
            }
        }
        Direction::E => {
            if node.steps < max_length && x < map[0].len() - 1 {
                nodes.push(Node {
                    direction: Direction::E,
                    steps: node.steps + 1,
                    idx: (x + 1, y),
                })
            }
            if node.steps >= min_length && y > 0 {
                nodes.push(Node {
                    direction: Direction::N,
                    steps: 1,
                    idx: (x, y - 1),
                })
            }
            if node.steps >= min_length && y < map.len() - 1 {
                nodes.push(Node {
                    direction: Direction::S,
                    steps: 1,
                    idx: (x, y + 1),
                })
            }
        }
    }

    nodes
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Part {
    P1,
    P2,
}

fn run_dijkstra(source: Node, map: Vec<Vec<u32>>, part: Part) -> u32 {
    let max_col = map[0].len();
    let max_row = map.len();

    dijkstra(
        source,
        Box::new(move |n| {
            match part {
                Part::P1 => get_neighbors(&map, n, 0, 3),
                Part::P2 => get_neighbors(&map, n, 4, 10),
            }
            .into_iter()
            .map(|n| {
                let cost = map[n.idx.1][n.idx.0];
                Neighbor {
                    node: n,
                    step_cost: cost,
                }
            })
            .collect_vec()
        }),
        Box::new(move |n| {
            if n.idx == (max_col - 1, max_row - 1) {
                if part == Part::P2 && n.steps < 4 {
                    return false;
                }

                return true;
            }
            false
        }),
    )
}

pub fn part1() {
    let map = read_input("inputs/day17.txt")
        .into_iter()
        .map(|x| {
            x.chars()
                .map(|x| (x as u8 - b'0') as u32)
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let result = std::cmp::min(
        run_dijkstra(
            Node {
                direction: Direction::E,
                steps: 0,
                idx: (0, 0),
            },
            map.clone(),
            Part::P1,
        ),
        run_dijkstra(
            Node {
                direction: Direction::S,
                steps: 0,
                idx: (0, 0),
            },
            map,
            Part::P1,
        ),
    );

    println!("Day 17 Part 1: {}", result);
}

pub fn part2() {
    let map = read_input("inputs/day17.txt")
        .into_iter()
        .map(|x| {
            x.chars()
                .map(|x| (x as u8 - b'0') as u32)
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let result = std::cmp::min(
        run_dijkstra(
            Node {
                direction: Direction::E,
                steps: 0,
                idx: (0, 0),
            },
            map.clone(),
            Part::P2,
        ),
        run_dijkstra(
            Node {
                direction: Direction::S,
                steps: 0,
                idx: (0, 0),
            },
            map,
            Part::P2,
        ),
    );

    println!("Day 17 Part 2: {}", result);
}
