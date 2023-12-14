use crate::utils::read_input;

// fn print_map(map: &Vec<Vec<i64>>) {
//     for row in map {
//         for col in row {
//             match col {
//                 0 => print!("."),
//                 1 => print!("#"),
//                 _ => print!("?"),
//             }
//         }
//         println!();
//     }
// }

fn convert_map(map: Vec<String>) -> Vec<Vec<i64>> {
    map.into_iter()
        .map(|x| {
            x.chars()
                .map(|x| if x == '.' { 0 } else { 1 })
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}

fn transpose(map: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut new_map = vec![];
    for i in 0..map[0].len() {
        let mut new_row = vec![];
        for j in 0..map.len() {
            new_row.push(map[j][i]);
        }
        new_map.push(new_row);
    }
    new_map
}

fn check_reflection_at_index(map: &Vec<Vec<i64>>, idx: usize) -> bool {
    let max_col = map[0].len();

    let span = std::cmp::min(max_col - idx - 2, idx);

    for i in 0..map.len() {
        for s in 0..=span {
            if map[i][idx - s] != map[i][idx + s + 1] {
                return false;
            }
        }
    }

    true
}

pub fn part1() {
    let patterns = read_input("inputs/day13.txt")
        .into_iter()
        .fold(Vec::new(), |mut acc, value| {
            if acc.is_empty() {
                acc.push(vec![]);
            }
            if value.is_empty() {
                acc.push(vec![]);
                return acc;
            }
            acc.last_mut().unwrap().push(value);
            acc
        })
        .into_iter()
        .map(convert_map)
        .map(|x| score(&x, 0))
        .sum::<i64>();

    println!("Day 13 Part 1: {}", patterns);
}

fn score(map: &Vec<Vec<i64>>, orig_score: i64) -> i64 {
    for i in 0..(map[0].len() - 1) {
        if check_reflection_at_index(map, i) && i as i64 + 1 != orig_score {
            return i as i64 + 1;
        }
    }

    // deep clone
    let mut map_clone = vec![];
    for row in map {
        map_clone.push(row.clone());
    }

    let x = transpose(map_clone);
    for i in 0..(x[0].len() - 1) {
        if check_reflection_at_index(&x, i) && (i as i64 + 1) * 100 != orig_score {
            return (i as i64 + 1) * 100;
        }
    }

    0
}

pub fn part2() {
    let patterns = read_input("inputs/day13.txt")
        .into_iter()
        .fold(Vec::new(), |mut acc, value| {
            if acc.is_empty() {
                acc.push(vec![]);
            }
            if value.is_empty() {
                acc.push(vec![]);
                return acc;
            }
            acc.last_mut().unwrap().push(value);
            acc
        })
        .into_iter()
        .map(convert_map)
        .map(|mut x| {
            let first_score = score(&x, 0);
            let max_col = x[0].len();
            let max_row = x.len();

            for i in 0..max_row {
                for j in 0..max_col {
                    x[i][j] = 1 - x[i][j];

                    let new_score = score(&x, first_score);
                    if new_score > 0 {
                        return new_score;
                    }

                    x[i][j] = 1 - x[i][j];
                }
            }

            first_score
        })
        .sum::<i64>();

    println!("Day 13 Part 2: {}", patterns);
}
