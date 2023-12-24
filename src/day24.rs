use itertools::Itertools;
use z3::{
    ast::{Ast, Int},
    Config, Context,
};

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

    println!("Day 24 Part 1: {}", intersections.len());
}

pub fn part2() {
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

    // i despise this
    let z3_config = Config::new();
    let z3_context = Context::new(&z3_config);
    let solver = z3::Solver::new(&z3_context);

    let target_x = Int::new_const(&z3_context, "target_x");
    let target_y = Int::new_const(&z3_context, "target_y");
    let target_z = Int::new_const(&z3_context, "target_z");
    let target_dx = Int::new_const(&z3_context, "target_dx");
    let target_dy = Int::new_const(&z3_context, "target_dy");
    let target_dz = Int::new_const(&z3_context, "target_dz");

    for (idx, d) in data.into_iter().enumerate() {
        let pos_x = Int::from_i64(&z3_context, d.pos.x);
        let pos_y = Int::from_i64(&z3_context, d.pos.y);
        let pos_z = Int::from_i64(&z3_context, d.pos.z);
        let speed_x = Int::from_i64(&z3_context, d.speed.dx);
        let speed_y = Int::from_i64(&z3_context, d.speed.dy);
        let speed_z = Int::from_i64(&z3_context, d.speed.dz);

        let t_name = format!("t_{}", idx);
        let t = Int::new_const(&z3_context, t_name);

        solver.assert(&(&pos_x + &speed_x * &t)._eq(&(&target_x + &target_dx * &t)));
        solver.assert(&(&pos_y + &speed_y * &t)._eq(&(&target_y + &target_dy * &t)));
        solver.assert(&(&pos_z + &speed_z * &t)._eq(&(&target_z + &target_dz * &t)));
    }

    solver.check();
    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&target_x).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&target_y).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&target_z).unwrap().as_i64().unwrap();
    println!("Day 24 Part 2: {}", x + y + z);
}
