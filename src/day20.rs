use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use crate::{math::lcm_of_vec, utils::read_input};

#[derive(Debug)]
enum ModuleState {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>),
    Output,
}

#[derive(Debug)]
struct Module {
    state: ModuleState,
    name: String,
    destinations: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

fn send_signal(module_map: &mut HashMap<String, Module>) -> (i64, i64, HashSet<String>) {
    // first one goes to broadcaster
    let mut low_signals = 1;
    let mut high_signals = 0;
    let mut cycles_from_vd = HashSet::new();

    let mut queue = VecDeque::new();
    let broadcaster_dest = module_map.get("broadcaster").unwrap().destinations.clone();
    for dest in broadcaster_dest {
        queue.push_back((Pulse::Low, "broadcaster".to_string(), dest));
        low_signals += 1;
    }

    while !queue.is_empty() {
        let current_module_id = queue.pop_front().unwrap();

        let current_module = module_map.get_mut(&current_module_id.2).unwrap();
        if let ModuleState::FlipFlop(state) = &mut current_module.state {
            if current_module_id.0 == Pulse::Low {
                if *state {
                    *state = false;
                    for dest in current_module.destinations.clone() {
                        queue.push_back((Pulse::Low, current_module_id.2.to_string(), dest));
                        low_signals += 1;
                    }
                } else {
                    *state = true;
                    for dest in current_module.destinations.clone() {
                        if &dest == "vd" {
                            cycles_from_vd.insert(current_module_id.2.clone());
                        }
                        queue.push_back((Pulse::High, current_module_id.2.to_string(), dest));
                        high_signals += 1;
                    }
                }
            }
        } else if let ModuleState::Conjunction(state) = &mut current_module.state {
            state.insert(current_module_id.1.clone(), current_module_id.0);

            if state.iter().all(|x| *x.1 == Pulse::High) {
                for dest in current_module.destinations.clone() {
                    queue.push_back((Pulse::Low, current_module_id.2.to_string(), dest));
                    low_signals += 1;
                }
            } else {
                for dest in current_module.destinations.clone() {
                    if &dest == "vd" {
                        cycles_from_vd.insert(current_module_id.2.clone());
                    }
                    queue.push_back((Pulse::High, current_module_id.2.to_string(), dest));
                    high_signals += 1;
                }
            }
        }
    }

    (low_signals, high_signals, cycles_from_vd)
}

pub fn part1() {
    let data = read_input("inputs/day20.txt")
        .into_iter()
        .map(|x| {
            let (key, destination) = x.split_once(" -> ").unwrap();
            let destination_nodes = destination.split(", ").map(|x| x.to_string()).collect_vec();

            if key == "broadcaster" {
                Module {
                    state: ModuleState::Broadcaster,
                    name: key.to_string(),
                    destinations: destination_nodes,
                }
            } else {
                match key.chars().next().unwrap() {
                    '%' => Module {
                        state: ModuleState::FlipFlop(false),
                        name: key[1..].to_string(),
                        destinations: destination_nodes,
                    },
                    '&' => Module {
                        state: ModuleState::Conjunction(HashMap::new()),
                        name: key[1..].to_string(),
                        destinations: destination_nodes,
                    },
                    _ => unreachable!(),
                }
            }
        })
        .collect_vec();

    let mut modules = HashMap::new();

    let mut reverse_mapping_initialization = HashMap::new();
    for module in data.into_iter() {
        for dest in module.destinations.iter() {
            if !reverse_mapping_initialization.contains_key(dest) {
                reverse_mapping_initialization.insert(dest.clone(), vec![]);
            }

            reverse_mapping_initialization
                .get_mut(dest)
                .unwrap()
                .push(module.name.clone());
        }
        modules.insert(module.name.clone(), module);
    }

    for (key, value) in reverse_mapping_initialization.into_iter() {
        let module = modules.get_mut(&key);

        if let Some(module) = module {
            match &mut module.state {
                ModuleState::Conjunction(state) => {
                    for val in value {
                        state.insert(val, Pulse::Low);
                    }
                }
                ModuleState::Broadcaster | ModuleState::Output | ModuleState::FlipFlop(_) => {}
            }
        } else {
            modules.insert(
                key.clone(),
                Module {
                    destinations: vec![],
                    name: key.clone(),
                    state: ModuleState::Output,
                },
            );
        }
    }

    let mut total_low = 0;
    let mut total_high = 0;

    for _ in 0..1000 {
        let (low_signals, high_signals, _) = send_signal(&mut modules);
        total_low += low_signals;
        total_high += high_signals;
    }

    println!("Day 20 Part 1: {}", total_low * total_high);
}

pub fn part2() {
    let data = read_input("inputs/day20.txt")
        .into_iter()
        .map(|x| {
            let (key, destination) = x.split_once(" -> ").unwrap();
            let destination_nodes = destination.split(", ").map(|x| x.to_string()).collect_vec();

            if key == "broadcaster" {
                Module {
                    state: ModuleState::Broadcaster,
                    name: key.to_string(),
                    destinations: destination_nodes,
                }
            } else {
                match key.chars().next().unwrap() {
                    '%' => Module {
                        state: ModuleState::FlipFlop(false),
                        name: key[1..].to_string(),
                        destinations: destination_nodes,
                    },
                    '&' => Module {
                        state: ModuleState::Conjunction(HashMap::new()),
                        name: key[1..].to_string(),
                        destinations: destination_nodes,
                    },
                    _ => unreachable!(),
                }
            }
        })
        .collect_vec();

    let mut modules = HashMap::new();

    let mut reverse_mapping_initialization = HashMap::new();
    for module in data.into_iter() {
        for dest in module.destinations.iter() {
            if !reverse_mapping_initialization.contains_key(dest) {
                reverse_mapping_initialization.insert(dest.clone(), vec![]);
            }

            reverse_mapping_initialization
                .get_mut(dest)
                .unwrap()
                .push(module.name.clone());
        }
        modules.insert(module.name.clone(), module);
    }

    for (key, value) in reverse_mapping_initialization.into_iter() {
        let module = modules.get_mut(&key);

        if let Some(module) = module {
            match &mut module.state {
                ModuleState::Conjunction(state) => {
                    for val in value {
                        state.insert(val, Pulse::Low);
                    }
                }
                ModuleState::Broadcaster | ModuleState::Output | ModuleState::FlipFlop(_) => {}
            }
        } else {
            modules.insert(
                key.clone(),
                Module {
                    destinations: vec![],
                    name: key.clone(),
                    state: ModuleState::Output,
                },
            );
        }
    }

    let mut button_presses = 0u64;

    // input observation - rx is conencted to conjunction which has 4 inputs.
    // find LCM of cycles of those 4 inputs and see if thats enough

    let mut cycle_lengths = HashMap::new();

    loop {
        button_presses += 1;
        let (_, _, hits) = send_signal(&mut modules);

        for h in hits {
            cycle_lengths.insert(h, button_presses);
        }

        if cycle_lengths.len() == 4 {
            break;
        }
    }

    println!(
        "Day 20 Part 2: {}",
        lcm_of_vec(cycle_lengths.values().copied().collect_vec().as_slice())
    );
}
