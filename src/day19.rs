use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::read_input;

#[derive(Debug, Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Debug, Clone)]
enum Param {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone)]
enum Op {
    Greater,
    Lesser,
}

#[derive(Debug, Clone)]
struct Condition {
    param: Param,
    value: usize,
    op: Op,
}

#[derive(Debug, Clone)]
struct Workflow {
    name: Label,
    conditions: Vec<Command>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Label {
    Workflow(String),
    Accept,
    Reject,
}

#[derive(Debug, Clone)]
enum Command {
    Direct(Label),
    Conditional(Condition, Label),
}

fn parse_input() -> (Vec<Workflow>, Vec<Part>) {
    let (raw_workflows, raw_items) = read_input("inputs/day19.txt")
        .into_iter()
        .fold(vec![], |mut acc, val| {
            if acc.is_empty() {
                acc.push(vec![]);
            }

            if val.is_empty() {
                acc.push(vec![]);
            } else {
                acc.last_mut().unwrap().push(val);
            }
            acc
        })
        .into_iter()
        .take(2)
        .collect_tuple::<(Vec<String>, Vec<String>)>()
        .unwrap();

    let workflows = raw_workflows
        .into_iter()
        .map(|w| {
            let name = w.split_once('{').unwrap().0;
            let name = match name {
                "A" => Label::Accept,
                "R" => Label::Reject,
                x => Label::Workflow(x.to_string()),
            };

            let conditions = w
                .split_once('{')
                .unwrap()
                .1
                .strip_suffix('}')
                .unwrap()
                .split(',')
                .map(|x| x.trim().to_string())
                .collect_vec();

            let mut final_conditions = vec![];

            for cond in conditions.into_iter() {
                if !cond.contains(':') {
                    if cond == "A" {
                        final_conditions.push(Command::Direct(Label::Accept));
                        continue;
                    } else if cond == "R" {
                        final_conditions.push(Command::Direct(Label::Reject));
                        continue;
                    } else {
                        final_conditions.push(Command::Direct(Label::Workflow(cond.to_string())));
                        continue;
                    }
                } else {
                    let (conditional_part, label) = cond.split_once(':').unwrap();
                    let label = match label {
                        "A" => Label::Accept,
                        "R" => Label::Reject,
                        x => Label::Workflow(x.to_string()),
                    };

                    let param_char = conditional_part.chars().next().unwrap();
                    let param = match param_char {
                        'x' => Param::X,
                        'm' => Param::M,
                        'a' => Param::A,
                        's' => Param::S,
                        _ => unreachable!(),
                    };

                    let op = match conditional_part.chars().nth(1).unwrap() {
                        '>' => Op::Greater,
                        '<' => Op::Lesser,
                        _ => unreachable!(),
                    };

                    let value = conditional_part
                        .chars()
                        .skip(2)
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();

                    final_conditions
                        .push(Command::Conditional(Condition { param, value, op }, label));
                }
            }
            Workflow {
                name,
                conditions: final_conditions,
            }
        })
        .collect_vec();

    let items = raw_items
        .into_iter()
        .map(|x| {
            let values = x
                .strip_prefix('{')
                .unwrap()
                .strip_suffix('}')
                .unwrap()
                .split(',');

            let mut final_part = Part {
                x: 0,
                m: 0,
                a: 0,
                s: 0,
            };
            for part in values {
                let (prop, val) = part.split_once('=').unwrap();
                let val = val.parse::<usize>().unwrap();
                match prop {
                    "x" => final_part.x = val,
                    "m" => final_part.m = val,
                    "a" => final_part.a = val,
                    "s" => final_part.s = val,
                    _ => unreachable!(),
                }
            }

            final_part
        })
        .collect_vec();
    (workflows, items)
}

fn simulate_workflow(part: &Part, workflow: &Workflow) -> Label {
    for command in workflow.conditions.iter() {
        match command {
            Command::Direct(x) => return x.clone(),
            Command::Conditional(cond, label) => {
                let value = match cond.param {
                    Param::X => part.x,
                    Param::M => part.m,
                    Param::A => part.a,
                    Param::S => part.s,
                };

                let result = match cond.op {
                    Op::Greater => value > cond.value,
                    Op::Lesser => value < cond.value,
                };

                if result {
                    return label.clone();
                }
            }
        }
    }

    unreachable!()
}

fn simulate_part(part: Part, workflows: &HashMap<Label, &Workflow>) -> Label {
    let mut current_label = Label::Workflow("in".to_string());

    while current_label != Label::Accept && current_label != Label::Reject {
        let workflow = workflows.get(&current_label).unwrap();
        current_label = simulate_workflow(&part, workflow);
    }

    current_label
}

pub fn part1() {
    let (workflows, parts) = parse_input();

    let mut workflows_map = HashMap::new();
    for workflow in workflows.iter() {
        workflows_map.insert(workflow.name.clone(), workflow);
    }

    let mut total_value = 0;
    for part in parts.into_iter() {
        let part_value = part.x + part.m + part.a + part.s;
        let result = simulate_part(part, &workflows_map);

        if result == Label::Accept {
            total_value += part_value;
        }
    }
    println!("Day 19 Part 1: {}", total_value);
}

#[derive(Debug, Clone)]
struct PartRange {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl PartRange {
    fn count_options(&self) -> usize {
        let x = self.x.1 - self.x.0;
        let m = self.m.1 - self.m.0;
        let a = self.a.1 - self.a.0;
        let s = self.s.1 - self.s.0;

        x * m * a * s
    }
}

fn workflows_accepted(
    workflow_map: &HashMap<Label, Workflow>,
    workflow: Workflow,
    range: PartRange,
) -> usize {
    match workflow.conditions.first().unwrap() {
        Command::Direct(x) => {
            if x == &Label::Accept {
                range.count_options()
            } else if x == &Label::Reject {
                // no bueno
                return 0;
            } else {
                let workflow = workflow_map.get(x).unwrap();
                return workflows_accepted(workflow_map, workflow.clone(), range);
            }
        }
        Command::Conditional(cond, label) => {
            // two options, either we meet condition, or we dont meet condition

            let mut meet_range = range.clone();
            let mut not_meet_range = range;

            match cond.param {
                Param::X => match cond.op {
                    Op::Greater => {
                        meet_range.x.0 = std::cmp::max(meet_range.x.0, cond.value + 1);
                        not_meet_range.x.1 = std::cmp::min(not_meet_range.x.1, cond.value + 1);
                    }
                    Op::Lesser => {
                        meet_range.x.1 = std::cmp::min(meet_range.x.1, cond.value);
                        not_meet_range.x.0 = std::cmp::max(not_meet_range.x.0, cond.value);
                    }
                },
                Param::M => match cond.op {
                    Op::Greater => {
                        meet_range.m.0 = std::cmp::max(meet_range.m.0, cond.value + 1);
                        not_meet_range.m.1 = std::cmp::min(not_meet_range.m.1, cond.value + 1);
                    }
                    Op::Lesser => {
                        meet_range.m.1 = std::cmp::min(meet_range.m.1, cond.value);
                        not_meet_range.m.0 = std::cmp::max(not_meet_range.m.0, cond.value);
                    }
                },
                Param::A => match cond.op {
                    Op::Greater => {
                        meet_range.a.0 = std::cmp::max(meet_range.a.0, cond.value + 1);
                        not_meet_range.a.1 = std::cmp::min(not_meet_range.a.1, cond.value + 1);
                    }
                    Op::Lesser => {
                        meet_range.a.1 = std::cmp::min(meet_range.a.1, cond.value);
                        not_meet_range.a.0 = std::cmp::max(not_meet_range.a.0, cond.value);
                    }
                },
                Param::S => match cond.op {
                    Op::Greater => {
                        meet_range.s.0 = std::cmp::max(meet_range.s.0, cond.value + 1);
                        not_meet_range.s.1 = std::cmp::min(not_meet_range.s.1, cond.value + 1);
                    }
                    Op::Lesser => {
                        meet_range.s.1 = std::cmp::min(meet_range.s.1, cond.value);
                        not_meet_range.s.0 = std::cmp::max(not_meet_range.s.0, cond.value);
                    }
                },
            }

            let mut modified_workflow = workflow.clone();
            modified_workflow.conditions = modified_workflow
                .conditions
                .into_iter()
                .skip(1)
                .collect_vec();

            let does_not_meet_score = if not_meet_range.count_options() > 0 {
                workflows_accepted(workflow_map, modified_workflow, not_meet_range)
            } else {
                0
            };
            let does_meet_score = if meet_range.count_options() > 0 {
                match label {
                    Label::Accept => meet_range.count_options(),
                    Label::Reject => 0,
                    Label::Workflow(_x) => {
                        let workflow = workflow_map.get(label).unwrap();
                        workflows_accepted(workflow_map, workflow.clone(), meet_range)
                    }
                }
            } else {
                0
            };
            does_not_meet_score + does_meet_score
        }
    }
}

pub fn part2() {
    let (workflows, _parts) = parse_input();

    let mut workflows_map = HashMap::new();
    for workflow in workflows.iter() {
        workflows_map.insert(workflow.name.clone(), workflow.clone());
    }

    let range = PartRange {
        x: (1, 4001),
        m: (1, 4001),
        a: (1, 4001),
        s: (1, 4001),
    };
    let workflow = workflows_map[&Label::Workflow("in".to_string())].clone();
    let score = workflows_accepted(&workflows_map, workflow, range);
    println!("Day 19 Part 2: {}", score);
}
