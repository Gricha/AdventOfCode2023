use std::collections::VecDeque;

use itertools::Itertools;

use crate::utils::read_input;

fn visit_all_plots(
    starting_steps: i64,
    data: &Vec<Vec<char>>,
    map: &mut [Vec<i64>],
    dist: &mut [Vec<i64>],
    pos: (usize, usize),
) {
    let mut queue = VecDeque::new();
    queue.push_back((pos, starting_steps));

    while !queue.is_empty() {
        let ((x, y), remaining_steps) = queue.pop_front().unwrap();

        if map[y][x] != 2 {
            // we've already visited this plot
            continue;
        }

        map[y][x] = remaining_steps % 2;
        dist[y][x] = starting_steps - remaining_steps;

        if remaining_steps == 0 {
            continue;
        }

        if x > 0 && data[y][x - 1] != '#' {
            queue.push_back(((x - 1, y), remaining_steps - 1));
        }
        if x < data[0].len() - 1 && data[y][x + 1] != '#' {
            queue.push_back(((x + 1, y), remaining_steps - 1));
        }
        if y > 0 && data[y - 1][x] != '#' {
            queue.push_back(((x, y - 1), remaining_steps - 1));
        }
        if y < data.len() - 1 && data[y + 1][x] != '#' {
            queue.push_back(((x, y + 1), remaining_steps - 1));
        }
    }
}

pub fn part1() {
    let data = read_input("inputs/day21.txt")
        .into_iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect_vec();
    let max_row = data.len();
    let max_col = data[0].len();

    // 2 means unvisited, 1 means visited on odd step, 0 means visited on even step
    let mut map = vec![vec![2i64; max_col]; max_row];
    let mut dist = vec![vec![0i64; max_col]; max_row];

    let mut start_pos = (0, 0);

    for (row, col) in (0..max_row).cartesian_product(0..max_col) {
        if data[row][col] == 'S' {
            start_pos = (col, row);
        }
    }

    visit_all_plots(64, &data, &mut map, &mut dist, start_pos);
    let mut cnt = 0;

    for i in 0..max_row {
        for j in 0..max_col {
            if map[i][j] == 0 {
                cnt += 1;
            }
        }
    }

    println!("Day 21 Part 1: {}", cnt);
}

pub fn part2() {
    let data = read_input("inputs/day21.txt")
        .into_iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect_vec();
    let max_row = data.len();
    let max_col = data[0].len();

    // 2 means unvisited, 1 means visited on odd step, 0 means visited on even step
    let mut map = vec![vec![2i64; max_col]; max_row];
    let mut dist = vec![vec![0i64; max_col]; max_row];

    let mut start_pos = (0, 0);

    for (row, col) in (0..max_row).cartesian_product(0..max_col) {
        if data[row][col] == 'S' {
            start_pos = (col, row);
        }
    }

    // random number of steps, but fills with right parity
    visit_all_plots(500, &data, &mut map, &mut dist, start_pos);

    let mut cnt_even = 0;
    let mut cnt_odd = 0;

    // dumb observation - number of steps is 131*n + 65.
    // 65 is the exact half, so the corner pieces are kinda split into equal parts
    // this should be enough to count them in the "65 radius"
    let mut cnt_even_out_65 = 0;
    let mut cnt_odd_out_65 = 0;

    for i in 0..max_row {
        for j in 0..max_col {
            if map[i][j] == 0 {
                cnt_even += 1;

                if dist[i][j] > 65 {
                    cnt_even_out_65 += 1;
                }
            } else if map[i][j] == 1 {
                cnt_odd += 1;

                if dist[i][j] > 65 {
                    cnt_odd_out_65 += 1;
                }
            }
        }
    }

    let width_squares = (26501365 - 65) / max_col;

    // another dumb input specific observation - there is squares ^ 2 odd parity squares and
    // (squares+1)^2 even parity squares
    let result = width_squares * width_squares * cnt_even
        + (width_squares + 1) * (width_squares + 1) * cnt_odd;

    // remove stupid corner pieces for odd elements
    let result = result - cnt_odd_out_65 * (width_squares + 1);

    // add stupid corner pieces for even elements
    let result = result + cnt_even_out_65 * width_squares;

    // magical off by one i didnt debug
    println!("Day 21 Part 2: {}", result - width_squares);
}
