use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::utils::read_input;

fn check_if_disjoint(
    graph: &HashMap<String, Vec<String>>,
    starting_node: String,
    removed_edges: HashSet<(String, String)>,
) -> Option<usize> {
    let mut visited = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back(starting_node);

    while let Some(node) = queue.pop_front() {
        if visited.contains(&node) {
            continue;
        }

        visited.insert(node.clone());

        for neighbor in graph.get(&node).unwrap() {
            if removed_edges.contains(&(node.clone(), neighbor.clone()))
                || removed_edges.contains(&(neighbor.clone(), node.clone()))
            {
                continue;
            }

            queue.push_back(neighbor.clone());
        }
    }

    if visited.len() == graph.len() {
        return None;
    } else {
        return Some(visited.len() * (graph.len() - visited.len()));
    }
}

pub fn part1() {
    let graph = read_input("inputs/day25.txt").into_iter().map(|x| {
        let (source, dests) = x.split_once(": ").unwrap();
        let source = source.to_string();
        let dests = dests
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();

        (source, dests)
    });

    let mut graph_complete = HashMap::<String, Vec<String>>::new();

    for (source, dests) in graph {
        for dest in dests {
            graph_complete
                .entry(source.clone())
                .or_default()
                .push(dest.clone());
            graph_complete
                .entry(dest.clone())
                .or_default()
                .push(source.clone());
        }
    }

    // dump into .dot file
    for (k, v) in graph_complete.iter() {
        for vv in v.iter() {
            println!("{} -- {}", k, vv);
        }
    }

    println!();

    let keys = graph_complete
        .iter()
        .filter(|(k, v)| {
            if v.len() > 4 {
                return false;
            }
            return true;
        })
        .map(|(x, _)| x.to_string())
        .collect::<HashSet<String>>();

    // sfdp -Goverlap_scaling=-16 -Tsvg graph.dot > graph.svg
    // shrug

    let mut removed_edges = HashSet::new();
    removed_edges.insert(("hvm".to_string(), "grd".to_string()));
    removed_edges.insert(("jmn".to_string(), "zfk".to_string()));
    removed_edges.insert(("kdc".to_string(), "pmn".to_string()));

    let starting_node = keys.iter().next().unwrap().clone();

    dbg!(check_if_disjoint(
        &graph_complete,
        starting_node,
        removed_edges
    ));
}

pub fn part2() {}
