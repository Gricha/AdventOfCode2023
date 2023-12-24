use std::collections::HashSet;

use itertools::Itertools;
use rust_decimal::Decimal;

use crate::utils::read_input;

#[derive(PartialEq, Eq, Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct Speed {
    dx: i64,
    dy: i64,
    dz: i64,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Particle {
    pos: Point,
    speed: Speed,
}

fn get_intersection(p1: &Particle, p2: &Particle) -> Option<(f64, f64)> {
    if p1 == p2 {
        return None;
    }

    if p1.speed == p2.speed {
        return None;
    }

    // hit time
    let d = (p2.speed.dy * p1.speed.dx - p2.speed.dx * p1.speed.dy) as f64;
    let t1 = (p2.speed.dx * (p1.pos.y - p2.pos.y) - p2.speed.dy * (p1.pos.x - p2.pos.x)) as f64 / d;
    let t2 = (p1.speed.dx * (p1.pos.y - p2.pos.y) - p1.speed.dy * (p1.pos.x - p2.pos.x)) as f64 / d;

    if t1 < 0.0 || t2 < 0.0 {
        return None;
    }

    let x = p1.pos.x as f64 + t1 * p1.speed.dx as f64;
    let y = p1.pos.y as f64 + t1 * p1.speed.dy as f64;

    Some((x, y))
}
pub fn part1() {
    let data = read_input("inputs/day24.txt")
        .into_iter()
        .map(|x| {
            let (coord, speed) = x.split_once(" @ ").unwrap();

            let (x, y, z) = coord
                .split(", ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();
            let (dx, dy, dz) = speed
                .split(", ")
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();

            Particle {
                pos: Point { x, y, z },
                speed: Speed { dx, dy, dz },
            }
        })
        .collect_vec();
    let mut intersections = vec![];
    for i in 0..data.len() {
        for j in i + 1..data.len() {
            let val = get_intersection(&data[i], &data[j]);
            if let Some((x, y)) = val {
                intersections.push((x, y, data[i].clone(), data[j].clone()));
            }
        }
    }

    let minimum = 200000000000000.0;
    let maximum = 400000000000000.0;
    let intersections = intersections
        .into_iter()
        .filter(|(x, y, _, _)| {
            if *x >= minimum && *x <= maximum && *y >= minimum && *y <= maximum {
                return true;
            }

            false
        })
        .collect_vec();

    dbg!(intersections.len());
}

pub fn part2() {}
