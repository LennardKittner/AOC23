use std::collections::HashMap;
use itertools::Itertools;
use num_integer::lcm;
use crate::days::day20::Rule::{Broadcast, Conjunction, FlipFlop};

#[derive(Clone, Debug)]
enum Rule {
    Broadcast{targets: Vec<String>},
    FlipFlop{state: bool, targets: Vec<String>},
    Conjunction{inputs: HashMap<String, bool>, targets: Vec<String>}
}

impl Rule {
    fn receive(&mut self, operation: &Operation) -> Vec<Operation> {
        match self {
            Broadcast{targets} => targets.iter().map(|t| Operation{from: operation.target.to_string() ,pulse: false, target: t.to_string()}).collect_vec(),
            FlipFlop{state, targets} => {
                if operation.pulse {
                    Vec::new()
                } else {
                    *state = !*state;
                    targets.iter().map(|t| Operation{from: operation.target.to_string(), pulse: *state, target: t.to_string()}).collect_vec()
                }
            }
            Conjunction{inputs, targets} => {
                inputs.insert(operation.from.to_string(), operation.pulse);
                let mut pulse = false;
                for input in inputs.values() {
                    if !input {
                        pulse = true;
                        break;
                    }
                }
                targets.iter().map(|t| Operation{from: operation.target.to_string(), pulse, target: t.to_string()}).collect_vec()
            }
        }
    }
}

#[derive(Debug)]
struct Operation {
    from: String,
    pulse: bool,
    target: String,
}

pub fn exec_day20_part1(input: &str) -> String {
    let (mut rules, _, _) = parse(input);

    let mut low_pulses = 0;
    let mut high_pulses = 0;
    let mut cache: HashMap<String, (usize, usize, HashMap<String, Rule>)> = HashMap::new();
    for _ in 0..1000 {
        let mut operations = vec![Operation {
            from: "button".to_string(),
            pulse: false,
            target: "roadcaster".to_string(),
        }];
        low_pulses += 1;
        let before = format!("{:?}", rules);
        let before_pulse = (low_pulses, high_pulses);
        if let Some(a) = cache.get(&before) {
            low_pulses += a.0;
            high_pulses += a.1;
            rules = a.2.clone();
            continue;
        }
        while !operations.is_empty() {
            let curr = operations.remove(0);
            if !rules.keys().contains(&curr.target) {
                continue;
            }
            let mut new_ops = rules.get_mut(&curr.target).unwrap().receive(&curr);
            if let Some(first) = new_ops.get(0) {
                if first.pulse {
                    high_pulses += new_ops.len();
                } else {
                    low_pulses += new_ops.len();
                }
            }
            operations.append(&mut new_ops);
        }
        cache.insert(before, (low_pulses - before_pulse.0, high_pulses - before_pulse.1, rules.clone()));
    }
    (low_pulses * high_pulses).to_string()
}

pub fn exec_day20_part2(input: &str) -> String {
    let (mut rules, input_to_rx, mut inputs_to_check) = parse(input);

    let mut cache: HashMap<String, HashMap<String, Rule>> = HashMap::new();
    let mut round = 0;
    loop {
        round += 1;
        let mut operations = vec![Operation {
            from: "button".to_string(),
            pulse: false,
            target: "roadcaster".to_string(),
        }];
        let before = format!("{:?}", rules);
        if let Some(a) = cache.get(&before) {
            rules = a.clone();
            continue;
        }
        while !operations.is_empty() {
            let curr = operations.remove(0);
            if inputs_to_check.contains_key(&curr.from) && curr.target == input_to_rx && curr.pulse && inputs_to_check[&curr.from] < 0 {
                inputs_to_check.insert(curr.from.to_string(), round);
            }
            if !rules.keys().contains(&curr.target) {
                continue;
            }
            let mut new_ops = rules.get_mut(&curr.target).unwrap().receive(&curr);
            operations.append(&mut new_ops);
        }
        if inputs_to_check.iter().filter(|(_, &n)| n < 0).count() == 0 {
            break;
        }
        cache.insert(before, rules.clone());
    }
    inputs_to_check.values().fold(1, |a, &d| lcm(a, d)).to_string()
}

fn parse(input: &str) -> (HashMap<String, Rule>, String, HashMap<String, i64>) {
    let mut rules = HashMap::new();
    let mut inputs: HashMap<String, HashMap<String, bool>> = HashMap::new();
    for line in input.lines() {
        let values = line.split(" -> ").collect_vec();
        rules.insert(values[0][1..values[0].len()].to_string(), match values[0].chars().next().unwrap() {
            '&' => {
                let targets = values[1].split(", ").map(|s| s.to_string()).collect_vec();
                update_inputs(&mut inputs, &values, &targets);
                Conjunction { inputs: HashMap::new(), targets }
            },
            '%' => {
                let targets = values[1].split(", ").map(|s| s.to_string()).collect_vec();
                update_inputs(&mut inputs, &values, &targets);
                FlipFlop { state: false, targets }
            },
            'b' => {
                let targets = values[1].split(", ").map(|s| s.to_string()).collect_vec();
                update_inputs(&mut inputs, &values, &targets);
                Broadcast { targets }
            },
            _ => panic!("how")
        });
    }

    let input_to_rx = inputs["rx"].keys().next().unwrap().to_string();
    let inputs_to_check: HashMap<String, i64> = inputs[&input_to_rx].keys().map(|k| (k.to_string(), -1)).collect();

    for (name, rule) in rules.iter_mut() {
        if let Some(map) = inputs.remove(name) {
            match rule {
                Broadcast { .. } => (),
                FlipFlop { .. } => (),
                Conjunction { inputs, .. } => *inputs = map,
            }
        }
    }
    (rules, input_to_rx, inputs_to_check)
}

fn update_inputs(inputs: &mut HashMap<String, HashMap<String, bool>>, values: &[&str], targets: &[String]) {
    for target in targets {
        if let Some(map) = inputs.get_mut(target) {
            map.insert(values[0][1..values[0].len()].to_string(), false);
        } else {
            let mut tmp: HashMap<String, bool> = HashMap::new();
            tmp.insert(values[0][1..values[0].len()].to_string(), false);
            inputs.insert(target.to_string(), tmp);
        }
    }
}
