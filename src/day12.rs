use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::read_input;

fn combinations(
    cache: &mut HashMap<(Vec<char>, Vec<i64>), i64>,
    record: &Vec<char>,
    numbers: &Vec<i64>,
) -> i64 {
    if cache.contains_key(&(record.clone(), numbers.clone())) {
        return *cache.get(&(record.clone(), numbers.clone())).unwrap();
    }

    let mut result = 0;

    if record.is_empty() {
        if numbers.is_empty() {
            // valid combination found
            return 1;
        } else {
            return 0;
        }
    }

    if record[0] == '.' || record[0] == '?' {
        // lets assume first digit is valid
        let new_record = record[1..].to_vec();
        result += combinations(cache, &new_record, numbers);
    }

    // now use the number as if it was an invalid one and part of the current number
    #[allow(clippy::collapsible_if)]
    if record[0] == '?' || record[0] == '#' {
        if !numbers.is_empty() {
            let curr_number = numbers[0] as usize;

            // try to slice entire block
            if record.len() >= curr_number && record[0..curr_number].iter().all(|x| *x != '.') {
                // validate that the number after the block isnt a #
                if record.len() == curr_number {
                    if numbers.len() > 1 {
                        result += 0;
                    } else {
                        result += 1;
                    }
                }

                if record.len() > curr_number && record[curr_number] != '#' {
                    if numbers.len() > 1 {
                        let new_record = record[(curr_number + 1)..].to_vec();
                        let new_numbers = numbers[1..].to_vec();
                        result += combinations(cache, &new_record, &new_numbers);
                    } else {
                        // probably 0 but could be 1, entry logic will solve it
                        let new_record = record[(curr_number + 1)..].to_vec();
                        result += combinations(cache, &new_record, &vec![]);
                    }
                }
            }
        }
    }

    cache.insert((record.clone(), numbers.clone()), result);
    result
}

pub fn part1() {
    let answer = read_input("inputs/day12.txt")
        .into_iter()
        .map(|x| {
            let (rec, numbers) = x.split_once(' ').unwrap();
            let numbers = numbers
                .split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            (rec.chars().clone().collect::<Vec<char>>(), numbers)
        })
        .map(|x| {
            let mut cache = HashMap::new();
            combinations(&mut cache, &x.0, &x.1)
        })
        .sum::<i64>();
    println!("Day 12 Part 1: {}", answer);
}

pub fn part2() {
    let answer = read_input("inputs/day12.txt")
        .into_iter()
        .map(|x| {
            let (rec, numbers) = x.split_once(' ').unwrap();
            let numbers = numbers
                .split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            (rec.chars().clone().collect::<Vec<char>>(), numbers)
        })
        .map(|(record, num)| {
            let mut new_record = vec![];
            let mut new_numbers = vec![];
            for _i in 0..5 {
                record.clone().into_iter().for_each(|x| new_record.push(x));
                new_record.push('?');
                num.clone().into_iter().for_each(|x| new_numbers.push(x));
            }
            let new_record = new_record.into_iter().rev().skip(1).rev().collect_vec();
            (new_record, new_numbers)
        })
        .map(|x| {
            let mut cache = HashMap::new();
            combinations(&mut cache, &x.0, &x.1)
        })
        .sum::<i64>();
    println!("Day 12 Part 1: {}", answer);
}
