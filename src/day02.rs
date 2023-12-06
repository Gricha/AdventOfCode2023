use crate::utils::read_input;

pub fn part1() {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    println!(
        "Day 2 Part 1: {}",
        read_input("inputs/day2.txt")
            .into_iter()
            .filter_map(|line| {
                let (idx, rest) = line.split_once(": ").unwrap();
                let idx = idx.split("Game ").nth(1).unwrap().parse::<u32>().unwrap();
                let is_larger = rest
                    .split(';')
                    .flat_map(|x| x.split(','))
                    .map(|x| x.trim())
                    .map(|val| {
                        let (value, color) = val.split_once(' ').unwrap();
                        match color {
                            "red" => value.parse::<u32>().unwrap() > max_red,
                            "green" => value.parse::<u32>().unwrap() > max_green,
                            "blue" => value.parse::<u32>().unwrap() > max_blue,
                            _ => panic!("Invalid color"),
                        }
                    })
                    .any(|x| x);
                if !is_larger {
                    Some(idx)
                } else {
                    None
                }
            })
            .sum::<u32>()
    )
}

pub fn part2() {
    println!(
        "Day 2 Part 2: {}",
        read_input("inputs/day2.txt")
            .into_iter()
            .map(|line| {
                let (_idx, rest) = line.split_once(": ").unwrap();
                let mut power_red = u32::MIN;
                let mut power_green = u32::MIN;
                let mut power_blue = u32::MIN;
                rest.split(';').flat_map(|x| x.split(',')).for_each(|x| {
                    let (value, color) = x.trim().split_once(' ').unwrap();
                    match color {
                        "red" => power_red = power_red.max(value.parse::<u32>().unwrap()),
                        "green" => power_green = power_green.max(value.parse::<u32>().unwrap()),
                        "blue" => power_blue = power_blue.max(value.parse::<u32>().unwrap()),
                        _ => panic!("Invalid color"),
                    }
                });

                power_blue * power_green * power_red
            })
            .sum::<u32>()
    )
}
