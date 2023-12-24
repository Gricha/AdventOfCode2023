use std::str::FromStr;

use itertools::Itertools;

use crate::utils::read_input;
const WIDTH: usize = 1000;

#[derive(Debug, Clone, Copy)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "R" => Direction::E,
            "L" => Direction::W,
            "U" => Direction::N,
            "D" => Direction::S,
            _ => unreachable!(),
        })
    }
}

fn flood_fill(map: &mut [[usize; WIDTH]; WIDTH], pos: (usize, usize)) {
    let mut queue = vec![pos];
    while let Some((x, y)) = queue.pop() {
        if map[y][x] == 0 {
            map[y][x] = 2;
            if x < WIDTH - 1 {
                queue.push((x + 1, y));
            }
            if x > 0 {
                queue.push((x - 1, y));
            }
            if y < WIDTH - 1 {
                queue.push((x, y + 1));
            }
            if y > 0 {
                queue.push((x, y - 1));
            }
        }
    }
}

pub fn part1() {
    let instructions = read_input("inputs/day18.txt")
        .into_iter()
        .map(|x| {
            let v = x.split_whitespace().collect_vec();
            let dir = v[0].parse::<Direction>().unwrap();
            let steps = v[1].parse::<usize>().unwrap();
            let color = v[2].to_string();
            (dir, steps, color)
        })
        .collect_vec();

    let mut pos = (500, 500);

    let mut map = [[0; WIDTH]; WIDTH];

    map[pos.1][pos.0] = 1;

    for (dir, steps, _color) in instructions {
        for _ in 0..steps {
            match dir {
                Direction::N => pos.1 -= 1,
                Direction::S => pos.1 += 1,
                Direction::E => pos.0 += 1,
                Direction::W => pos.0 -= 1,
            }
            map[pos.1][pos.0] = 1;
        }
    }
    flood_fill(&mut map, (0, 0));

    let mut count = 0;
    for y in 0..WIDTH {
        for x in 0..WIDTH {
            if map[y][x] != 2 {
                count += 1;
            }
        }
    }
    println!("Day 18 Part 1: {}", count);
}

fn calculate_area(points: &[(i64, i64)]) -> i64 {
    let mut sum1 = 0;
    let mut sum2 = 0;

    for i in 0..points.len() {
        let current = &points[i];
        let next = &points[(i + 1) % points.len()];

        sum1 += current.0 * next.1;
        sum2 += current.1 * next.0;
    }

    (sum1 - sum2).abs() / 2 + 1
}

pub fn part2() {
    let instructions = read_input("inputs/day18.txt")
        .into_iter()
        .map(|x| {
            let v = x.split_whitespace().collect_vec();
            let color = v[2].to_string();

            let number = usize::from_str_radix(&color[2..7], 16).unwrap();
            let dir = match color.chars().nth(7).unwrap() {
                '0' => Direction::E,
                '1' => Direction::S,
                '2' => Direction::W,
                '3' => Direction::N,
                _ => unreachable!(),
            };

            (dir, number)
        })
        .collect_vec();

    let mut points = vec![];

    let mut pos = (500i64, 500i64);
    points.push(pos);

    for (dir, steps) in instructions.iter() {
        match dir {
            Direction::N => pos.1 -= *steps as i64,
            Direction::S => pos.1 += *steps as i64,
            Direction::E => pos.0 += *steps as i64,
            Direction::W => pos.0 -= *steps as i64,
        }
        points.push(pos);
    }

    let sum_edge = instructions.iter().map(|(_, x)| *x as i64).sum::<i64>();

    println!("Day 18 Part 2: {}", calculate_area(&points) + sum_edge / 2);
}
