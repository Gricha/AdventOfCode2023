use std::ops::Range;

use itertools::Itertools;
use rangemap::RangeMap;

use crate::utils::read_input;

fn extract_mapping(chunk: &[String]) -> RangeMap<u64, (u64, u64)> {
    let mut mapping = RangeMap::new();
    for line in chunk.iter().skip(1) {
        let values = line.split_whitespace().map(|x| x.parse::<u64>().unwrap());
        let (start_destination, start_source, range_length) = values.collect_tuple().unwrap();
        mapping.insert(
            start_source..(start_source + range_length),
            (start_source, start_destination),
        );
    }
    mapping
}

pub fn part1() {
    let lines = read_input("inputs/day5.txt")
        .into_iter()
        .fold(Vec::new(), |mut acc, value| {
            if acc.is_empty() {
                acc.push(vec![value])
            } else if value.is_empty() {
                acc.push(vec![])
            } else {
                acc.last_mut().unwrap().push(value)
            }
            acc
        });

    let seeds = lines[0][0]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect_vec();
    let seed_to_soil = extract_mapping(&lines[1]);
    let soil_to_fertilizer = extract_mapping(&lines[2]);
    let fertilizer_to_water = extract_mapping(&lines[3]);
    let water_to_light = extract_mapping(&lines[4]);
    let light_to_temperature = extract_mapping(&lines[5]);
    let temperature_to_humidity = extract_mapping(&lines[6]);
    let humidity_to_location = extract_mapping(&lines[7]);

    let generic_mapper = |mapping: RangeMap<u64, (u64, u64)>| {
        Box::new(move |number: u64| {
            let lookup = &mapping.get(&number);
            match lookup {
                Some((start_origin, start_destination)) => {
                    start_destination + (number - start_origin)
                }
                None => number,
            }
        })
    };

    let soil = generic_mapper(seed_to_soil);
    let fertilizer = generic_mapper(soil_to_fertilizer);
    let water = generic_mapper(fertilizer_to_water);
    let light = generic_mapper(water_to_light);
    let temperature = generic_mapper(light_to_temperature);
    let humidity = generic_mapper(temperature_to_humidity);
    let location = generic_mapper(humidity_to_location);

    let lowest_seed_location = seeds
        .iter()
        .map(|x| location(humidity(temperature(light(water(fertilizer(soil(*x))))))))
        .min()
        .unwrap();
    println!("Day 5 Part 1: {}", lowest_seed_location)
}

fn intersect_ranges<T: std::cmp::Ord + Copy>(range1: &Range<T>, range2: &Range<T>) -> Range<T> {
    let start = std::cmp::max(range1.start, range2.start);
    let end = std::cmp::min(range1.end, range2.end);

    if start < end {
        start..end
    } else {
        panic!("eh")
    }
}

pub fn part2() {
    let lines = read_input("inputs/day5.txt")
        .into_iter()
        .fold(Vec::new(), |mut acc, value| {
            if acc.is_empty() {
                acc.push(vec![value])
            } else if value.is_empty() {
                acc.push(vec![])
            } else {
                acc.last_mut().unwrap().push(value)
            }
            acc
        });

    let seeds: Vec<(u64, u64)> = lines[0][0]
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .chunks(2)
        .into_iter()
        .map(|c| c.collect_tuple().unwrap())
        .collect_vec();
    let seed_to_soil = extract_mapping(&lines[1]);
    let soil_to_fertilizer = extract_mapping(&lines[2]);
    let fertilizer_to_water = extract_mapping(&lines[3]);
    let water_to_light = extract_mapping(&lines[4]);
    let light_to_temperature = extract_mapping(&lines[5]);
    let temperature_to_humidity = extract_mapping(&lines[6]);
    let humidity_to_location = extract_mapping(&lines[7]);

    let generic_mapper = |mapping: RangeMap<u64, (u64, u64)>| {
        Box::new(move |ranges: Vec<Range<u64>>| {
            let mut new_ranges = Vec::new();

            for range in ranges {
                for dummy_overlap in mapping.overlapping(&range) {
                    // we need a proper overlap with this range
                    let overlap = intersect_ranges(&range, dummy_overlap.0);

                    let (start_origin, start_destination) = dummy_overlap.1;
                    let offset = overlap.start - start_origin;
                    let overlapping_range_size = overlap.end - overlap.start;
                    new_ranges.push(
                        (start_destination + offset)
                            ..(start_destination + offset + overlapping_range_size),
                    );
                }

                for gap in mapping.gaps(&range) {
                    new_ranges.push(gap);
                }
            }
            new_ranges
        })
    };

    let soil = generic_mapper(seed_to_soil);
    let fertilizer = generic_mapper(soil_to_fertilizer);
    let water = generic_mapper(fertilizer_to_water);
    let light = generic_mapper(water_to_light);
    let temperature = generic_mapper(light_to_temperature);
    let humidity = generic_mapper(temperature_to_humidity);
    let location = generic_mapper(humidity_to_location);

    #[allow(clippy::single_range_in_vec_init)]
    let lowest_seed_location = seeds
        .iter()
        .map(|x| {
            location(humidity(temperature(light(water(fertilizer(soil(vec![
                (x.0)..(x.0 + x.1),
            ])))))))
            .into_iter()
            .map(|x| x.start)
            .min()
            .unwrap()
        })
        .collect_vec()
        .into_iter()
        .min()
        .unwrap();
    println!("Day 5 Part 2: {}", lowest_seed_location)
}
