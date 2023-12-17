use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::utils::read_input;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct NodeWithCost {
    node: Node,
    cost: u32,
}

impl PartialOrd for NodeWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.cmp(&other.cost).reverse())
    }
}

impl Ord for NodeWithCost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

fn get_neighbors(map: &Vec<Vec<u32>>, node: &Node) -> Vec<Node> {
    let mut nodes = vec![];

    let (x, y) = node.idx;

    match node.direction {
        Direction::N => {
            if node.steps < 3 && y > 0 {
                nodes.push(Node {
                    direction: Direction::N,
                    steps: node.steps + 1,
                    idx: (x, y - 1),
                })
            }
            if x > 0 {
                nodes.push(Node {
                    direction: Direction::W,
                    steps: 1,
                    idx: (x - 1, y),
                })
            }
            if x < map[0].len() - 1 {
                nodes.push(Node {
                    direction: Direction::E,
                    steps: 1,
                    idx: (x + 1, y),
                })
            }
        }
        Direction::S => {
            if node.steps < 3 && y < map.len() - 1 {
                nodes.push(Node {
                    direction: Direction::S,
                    steps: node.steps + 1,
                    idx: (x, y + 1),
                })
            }
            if x > 0 {
                nodes.push(Node {
                    direction: Direction::W,
                    steps: 1,
                    idx: (x - 1, y),
                })
            }
            if x < map[0].len() - 1 {
                nodes.push(Node {
                    direction: Direction::E,
                    steps: 1,
                    idx: (x + 1, y),
                })
            }
        }

        Direction::W => {
            if node.steps < 3 && x > 0 {
                nodes.push(Node {
                    direction: Direction::W,
                    steps: node.steps + 1,
                    idx: (x - 1, y),
                })
            }
            if y > 0 {
                nodes.push(Node {
                    direction: Direction::N,
                    steps: 1,
                    idx: (x, y - 1),
                })
            }
            if y < map.len() - 1 {
                nodes.push(Node {
                    direction: Direction::S,
                    steps: 1,
                    idx: (x, y + 1),
                })
            }
        }
        Direction::E => {
            if node.steps < 3 && x < map[0].len() - 1 {
                nodes.push(Node {
                    direction: Direction::E,
                    steps: node.steps + 1,
                    idx: (x + 1, y),
                })
            }
            if y > 0 {
                nodes.push(Node {
                    direction: Direction::N,
                    steps: 1,
                    idx: (x, y - 1),
                })
            }
            if y < map.len() - 1 {
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

fn get_neighbors_but_big(map: &Vec<Vec<u32>>, node: &Node) -> Vec<Node> {
    let mut nodes = vec![];

    let (x, y) = node.idx;

    match node.direction {
        Direction::N => {
            if node.steps < 10 && y > 0 {
                nodes.push(Node {
                    direction: Direction::N,
                    steps: node.steps + 1,
                    idx: (x, y - 1),
                })
            }
            if node.steps >= 4 && x > 0 {
                nodes.push(Node {
                    direction: Direction::W,
                    steps: 1,
                    idx: (x - 1, y),
                })
            }
            if node.steps >= 4 && x < map[0].len() - 1 {
                nodes.push(Node {
                    direction: Direction::E,
                    steps: 1,
                    idx: (x + 1, y),
                })
            }
        }
        Direction::S => {
            if node.steps < 10 && y < map.len() - 1 {
                nodes.push(Node {
                    direction: Direction::S,
                    steps: node.steps + 1,
                    idx: (x, y + 1),
                })
            }
            if node.steps >= 4 && x > 0 {
                nodes.push(Node {
                    direction: Direction::W,
                    steps: 1,
                    idx: (x - 1, y),
                })
            }
            if node.steps >= 4 && x < map[0].len() - 1 {
                nodes.push(Node {
                    direction: Direction::E,
                    steps: 1,
                    idx: (x + 1, y),
                })
            }
        }

        Direction::W => {
            if node.steps < 10 && x > 0 {
                nodes.push(Node {
                    direction: Direction::W,
                    steps: node.steps + 1,
                    idx: (x - 1, y),
                })
            }
            if node.steps >= 4 && y > 0 {
                nodes.push(Node {
                    direction: Direction::N,
                    steps: 1,
                    idx: (x, y - 1),
                })
            }
            if node.steps >= 4 && y < map.len() - 1 {
                nodes.push(Node {
                    direction: Direction::S,
                    steps: 1,
                    idx: (x, y + 1),
                })
            }
        }
        Direction::E => {
            if node.steps < 10 && x < map[0].len() - 1 {
                nodes.push(Node {
                    direction: Direction::E,
                    steps: node.steps + 1,
                    idx: (x + 1, y),
                })
            }
            if node.steps >= 4 && y > 0 {
                nodes.push(Node {
                    direction: Direction::N,
                    steps: 1,
                    idx: (x, y - 1),
                })
            }
            if node.steps >= 4 && y < map.len() - 1 {
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

fn dijkstra(source: Node, map: &Vec<Vec<u32>>, part: Part) -> u32 {
    let max_col = map[0].len();
    let max_row = map.len();

    let mut visited = HashSet::<Node>::new();
    let mut queue = BinaryHeap::<NodeWithCost>::new();
    let mut dist = HashMap::<Node, u32>::new();
    dist.insert(source, 0u32);

    queue.push(NodeWithCost {
        node: source,
        cost: 0,
    });

    while let Some(NodeWithCost { node, cost }) = queue.pop() {
        if !visited.insert(node) {
            continue;
        }

        if node.idx == (max_col - 1, max_row - 1) {
            if part == Part::P2 && node.steps < 4 {
                continue;
            }

            return cost;
        }

        let neighbors = match part {
            Part::P1 => get_neighbors(map, &node),
            Part::P2 => get_neighbors_but_big(map, &node),
        };

        for neighbor in neighbors {
            let new_cost = cost + map[neighbor.idx.1][neighbor.idx.0];

            if !dist.contains_key(&neighbor) || new_cost < *dist.get(&neighbor).unwrap() {
                // println!("Inserting cost {} for {:?}", new_cost, neighbor);
                dist.insert(neighbor, new_cost);
                queue.push(NodeWithCost {
                    node: neighbor,
                    cost: new_cost,
                });
            }
        }
    }

    unreachable!()
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
        dijkstra(
            Node {
                direction: Direction::E,
                steps: 0,
                idx: (0, 0),
            },
            &map,
            Part::P1,
        ),
        dijkstra(
            Node {
                direction: Direction::S,
                steps: 0,
                idx: (0, 0),
            },
            &map,
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
        dijkstra(
            Node {
                direction: Direction::E,
                steps: 0,
                idx: (0, 0),
            },
            &map,
            Part::P2,
        ),
        dijkstra(
            Node {
                direction: Direction::S,
                steps: 0,
                idx: (0, 0),
            },
            &map,
            Part::P2,
        ),
    );

    println!("Day 17 Part 2: {}", result);
}
