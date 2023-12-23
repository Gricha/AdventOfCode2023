use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::RangeInclusive,
};

use itertools::Itertools;

use crate::utils::read_input;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Block {
    start: Point,
    end: Point,
}

fn ranges_intersect<T>(range1: &RangeInclusive<T>, range2: &RangeInclusive<T>) -> bool
where
    T: Ord,
{
    range1.start() <= range2.end() && range2.start() <= range1.end()
}

fn planes_intersect(plane1: &Block, plane2: &Block) -> bool {
    ranges_intersect(
        &(plane1.start.x..=plane1.end.x),
        &(plane2.start.x..=plane2.end.x),
    ) && ranges_intersect(
        &(plane1.start.y..=plane1.end.y),
        &(plane2.start.y..=plane2.end.y),
    )
}

fn drop_blocks(blocks: &mut [Block]) -> (HashMap<usize, Vec<usize>>, HashMap<usize, Vec<usize>>) {
    let mut mapping = HashMap::<(i64, i64), i64>::new();
    // uses indexes of blocks in blocks array
    let mut above = HashMap::<usize, Vec<usize>>::new();
    let mut below = HashMap::<usize, Vec<usize>>::new();

    for mut block in blocks.iter_mut() {
        let x_range = block.start.x..=block.end.x;
        let y_range = block.start.y..=block.end.y;
        let all_coords = x_range.cartesian_product(y_range).collect_vec();

        let mut max_z_we_can_drop = 1;

        for (x, y) in all_coords.clone() {
            if mapping.contains_key(&(x, y)) {
                let current_max_z = mapping[&(x, y)];
                max_z_we_can_drop = std::cmp::max(max_z_we_can_drop, current_max_z + 1);
            }
        }

        let new_z_for_block = max_z_we_can_drop;
        let new_z_end = new_z_for_block + (block.end.z - block.start.z);
        block.start.z = new_z_for_block;
        block.end.z = new_z_end;

        for (x, y) in all_coords {
            mapping.insert((x, y), new_z_end);
        }
    }

    for (idx, block) in blocks.iter().enumerate() {
        let mut blocks_above = Vec::new();
        let mut blocks_below = Vec::new();

        for (idx2, block2) in blocks.iter().enumerate() {
            if idx == idx2 {
                continue;
            }

            if planes_intersect(block, block2) {
                if block2.start.z == block.end.z + 1 {
                    blocks_above.push(idx2);
                } else if block2.end.z == block.start.z - 1 {
                    blocks_below.push(idx2);
                }
            }
        }

        above.insert(idx, blocks_above);
        below.insert(idx, blocks_below);
    }

    (above, below)
}

pub fn part1() {
    let data = read_input("inputs/day22.txt")
        .into_iter()
        .map(|x| {
            let (start, end) = x.split_once('~').unwrap();
            let (sx, sy, sz) = start
                .split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();
            let (ex, ey, ez) = end
                .split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();

            Block {
                start: Point {
                    x: std::cmp::min(sx, ex),
                    y: std::cmp::min(sy, ey),
                    z: std::cmp::min(sz, ez),
                },
                end: Point {
                    x: std::cmp::max(sx, ex),
                    y: std::cmp::max(sy, ey),
                    z: std::cmp::max(sz, ez),
                },
            }
        })
        .sorted_by(|a, b| a.start.z.cmp(&b.start.z))
        .collect_vec();

    let mut coordinates = data;
    let (above, below) = drop_blocks(&mut coordinates);

    let mut safe_to_remove = 0;
    for idx in 0..coordinates.len() {
        if above[&idx].is_empty() {
            safe_to_remove += 1;
            continue;
        }

        let aboves = &above[&idx];

        let mut safe = true;
        for above in aboves {
            if below[above].len() <= 1 {
                safe = false;
                break;
            }
        }

        if safe {
            safe_to_remove += 1;
        }
    }
    println!("Day 22 Part 1: {}", safe_to_remove);
}

pub fn part2() {
    let data = read_input("inputs/day22.txt")
        .into_iter()
        .map(|x| {
            let (start, end) = x.split_once('~').unwrap();
            let (sx, sy, sz) = start
                .split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();
            let (ex, ey, ez) = end
                .split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();

            Block {
                start: Point {
                    x: std::cmp::min(sx, ex),
                    y: std::cmp::min(sy, ey),
                    z: std::cmp::min(sz, ez),
                },
                end: Point {
                    x: std::cmp::max(sx, ex),
                    y: std::cmp::max(sy, ey),
                    z: std::cmp::max(sz, ez),
                },
            }
        })
        .sorted_by(|a, b| a.start.z.cmp(&b.start.z))
        .collect_vec();

    let mut coordinates = data;
    let (above, below) = drop_blocks(&mut coordinates);

    let mut total_falls = 0;
    for idx in 0..coordinates.len() {
        let mut queue = VecDeque::new();
        let mut falling = HashSet::new();
        queue.push_back(idx);
        falling.insert(idx);

        while !queue.is_empty() {
            let current_idx = queue.pop_front().unwrap();
            let aboves = &above[&current_idx];

            for above in aboves {
                if below[above].iter().all(|x| falling.contains(x)) {
                    queue.push_back(*above);
                    falling.insert(*above);
                }
            }
        }
        total_falls += falling.len() - 1;
    }
    println!("Day 22 Part 2: {}", total_falls);
}
