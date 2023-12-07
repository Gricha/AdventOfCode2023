use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::read_input;

#[derive(Debug)]
struct Hand {
    cards: Vec<u32>,
}

impl Hand {
    fn types(&self) -> u32 {
        let mut set = HashSet::new();
        for card in self.cards.iter() {
            set.insert(*card);
        }

        let mut sizing = set.len() as u32;

        if set.contains(&1) {
            sizing -= 1;
        }

        std::cmp::max(sizing, 1)
    }

    fn multiplier(&self) -> u32 {
        let mut multiset = HashMap::<u32, u32>::new();
        for card in self.cards.iter() {
            if multiset.contains_key(card) {
                let val = *multiset.get(card).unwrap();
                multiset.insert(*card, val + 1);
            } else {
                multiset.insert(*card, 1);
            }
        }

        let jokers = multiset.remove(&1);

        if let Some(jokers) = jokers {
            let mut values = multiset.values().copied().sorted().collect_vec();

            if values.is_empty() {
                values.push(0);
            }

            let size = values.len();
            values[size - 1] += jokers;
            return values.into_iter().product::<u32>();
        }

        multiset.into_values().product::<u32>()
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.types().cmp(&other.types()) {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => match self.multiplier().cmp(&other.multiplier()) {
                Ordering::Greater => Ordering::Less,
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => {
                    for i in 0..5 {
                        #[allow(clippy::comparison_chain)]
                        if self.cards[i] > other.cards[i] {
                            return Ordering::Greater;
                        } else if self.cards[i] < other.cards[i] {
                            return Ordering::Less;
                        }
                    }

                    Ordering::Equal
                }
            },
        }
    }
}

pub fn part1() {
    let hands = read_input("inputs/day7.txt")
        .into_iter()
        .map(|x| {
            x.split(' ')
                .map(|x| x.to_string())
                .collect_tuple::<(String, String)>()
                .unwrap()
        })
        .map(|(hand, bid)| {
            let h = hand.trim().chars();
            let mut cards = Vec::new();
            for c in h.into_iter().take(5) {
                if c.is_ascii_digit() {
                    cards.push(c.to_digit(10).unwrap());
                } else {
                    let val = match c {
                        'T' => 10,
                        'J' => 11,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        _ => panic!("Invalid card"),
                    };
                    cards.push(val);
                }
            }
            (Hand { cards }, bid.parse::<u32>().unwrap())
        })
        .sorted()
        .enumerate()
        .map(|(idx, (_, bid))| (idx as u32 + 1) * bid)
        .sum::<u32>();
    println!("Day 7 Part 1: {}", hands);
}

pub fn part2() {
    let hands = read_input("inputs/day7.txt")
        .into_iter()
        .map(|x| {
            x.split(' ')
                .map(|x| x.to_string())
                .collect_tuple::<(String, String)>()
                .unwrap()
        })
        .map(|(hand, bid)| {
            let h = hand.trim().chars();
            let mut cards = Vec::new();
            for c in h.into_iter().take(5) {
                if c.is_ascii_digit() {
                    cards.push(c.to_digit(10).unwrap());
                } else {
                    let val = match c {
                        'T' => 10,
                        'J' => 1,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        _ => panic!("Invalid card"),
                    };
                    cards.push(val);
                }
            }
            (Hand { cards }, bid.parse::<u32>().unwrap())
        })
        .sorted()
        .enumerate()
        .map(|(idx, (_, bid))| (idx as u32 + 1) * bid)
        .sum::<u32>();
    println!("Day 7 Part 2: {}", hands);
}
