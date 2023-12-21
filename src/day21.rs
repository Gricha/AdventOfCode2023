use std::collections::VecDeque;

use itertools::Itertools;

use crate::utils::read_input;

fn visit_all_plots(
    remaining_steps: i64,
    data: &Vec<Vec<char>>,
    map: &mut Vec<Vec<i64>>,
    pos: (usize, usize),
) {
    let mut queue = VecDeque::new();
    queue.push_back((pos, remaining_steps));

    while !queue.is_empty() {
        let ((x, y), remaining_steps) = queue.pop_front().unwrap();

        if map[y][x] != 2 {
            // we've already visited this plot
            continue;
        }

        println!("Visiting {:?} with {remaining_steps} steps left", (x, y));

        map[y][x] = remaining_steps % 2;

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

    // if map[y][x] != 2 {
    //     // we've already visited this plot
    //     return;
    // }

    // println!("Visiting {pos:?} with {remaining_steps} steps left",);

    // map[y][x] = remaining_steps % 2;

    // if remaining_steps == 0 {
    //     return;
    // }

    // if x > 0 && data[y][x - 1] != '#' {
    //     visit_plot(remaining_steps - 1, data, map, (x - 1, y));
    // }
    // if x < data[0].len() - 1 && data[y][x + 1] != '#' {
    //     visit_plot(remaining_steps - 1, data, map, (x + 1, y));
    // }
    // if y > 0 && data[y - 1][x] != '#' {
    //     visit_plot(remaining_steps - 1, data, map, (x, y - 1));
    // }
    // if y < data.len() - 1 && data[y + 1][x] != '#' {
    //     visit_plot(remaining_steps - 1, data, map, (x, y + 1));
    // }
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

    let mut start_pos = (0, 0);

    for (row, col) in (0..max_row).cartesian_product(0..max_col) {
        if data[row][col] == 'S' {
            start_pos = (col, row);
        }
    }

    visit_all_plots(64, &data, &mut map, start_pos);
    let mut cnt = 0;

    for i in 0..max_row {
        for j in 0..max_col {
            if map[i][j] == 0 {
                cnt += 1;
            }
            if data[i][j] == '#' {
                print!("#");
            } else if map[i][j] == 2 {
                print!(".")
            } else if map[i][j] == 1 {
                print!("-");
            } else if data[i][j] == 'S' {
                print!("S")
            } else {
                print!("{}", map[i][j]);
            }
        }
        println!();
    }

    dbg!(cnt);
}

pub fn part2() {}
