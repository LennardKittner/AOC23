use std::collections::HashMap;
use std::ops::Range;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

#[derive(Debug)]
struct Rule {
    branches: Vec<Branch>,
    default: String,
}

impl Rule {
    fn apply(&self, part: &Part) -> String {
        let mut result = self.default.to_string();
        for branch in &self.branches {
            let comp = match &branch.parameter {
                b'x' => part.x,
                b'm' => part.m,
                b'a' => part.a,
                b's' => part.s,
                _ => panic!("how"),
            };
            if (branch.greater && comp > branch.constant) || (!branch.greater && comp < branch.constant) {
                result = branch.target.to_string();
                break;
            }
        }
        result
    }
}

#[derive(Debug)]
struct Branch {
    parameter: u8,
    greater: bool,
    constant: i64,
    target: String,
}

#[derive(Debug)]
struct Rule2 {
    branches: Vec<Branch>,
    default: String,
}

pub fn exec_day19_part1(input: &str) -> String {
    let regex_rule_outer = Regex::new(r"([a-zA-Z]+)\{(.*),([a-zA-Z]+)\}").unwrap();
    let regex_branch = Regex::new(r"([xmas])+([<>])(\d+):([a-zA-Z]+)").unwrap();
    let input = input.split("\n\n").collect_vec();
    let mut rules  = HashMap::new();
    for rule in input[0].lines() {
        let cap = regex_rule_outer.captures(rule).unwrap();
        let cap2 = regex_branch.captures_iter(cap.get(2).unwrap().as_str());
        let branches = cap2.map(|m| {
            Branch {
                parameter: m.get(1).unwrap().as_str().as_bytes()[0],
                greater: m.get(2).unwrap().as_str() == ">",
                constant: m.get(3).unwrap().as_str().parse().unwrap(),
                target: m.get(4).unwrap().as_str().to_string(),
            }
        }).collect_vec();
        rules.insert(cap.get(1).unwrap().as_str().to_string(), Rule { branches, default: cap.get(3).unwrap().as_str().to_string() });
    }

    let regex_parts = Regex::new(r"\{x=(?P<x>\d+),m=(?P<m>\d+),a=(?P<a>\d+),s=(?P<s>\d+)\}").unwrap();
    let parts = input[1].lines().map(|part| {
        let cap = regex_parts.captures(part).unwrap();
        Part {
            x: cap.name("x").unwrap().as_str().parse().unwrap(),
            m: cap.name("m").unwrap().as_str().parse().unwrap(),
            a: cap.name("a").unwrap().as_str().parse().unwrap(),
            s: cap.name("s").unwrap().as_str().parse().unwrap(),
        }
    }).collect_vec();

    let mut result = 0;
    let mut curr;
    for part in parts {
        curr = "in".to_string();
        if loop {
            curr = rules.get(&curr).unwrap().apply(&part);
            if curr == "R" {
                break false
            }
            if curr == "A" {
                break true
            }
        } {
            result += part.x + part.m + part.a + part.s;
        }
    }
    result.to_string()
}

#[derive(Debug, Clone)]
struct Part2 {
    x: Range<i64>,
    m: Range<i64>,
    a: Range<i64>,
    s: Range<i64>,
    state: String,
}

fn new_range(range: &Range<i64>, val: i64, greater: bool) -> Option<Range<i64>> {
    if greater && range.end > val+1 {
        Some(range.start.max(val+1)..range.end)
    } else if !greater && range.start < val {
        Some(range.start..range.end.min(val))
    } else {
        None
    }
}


impl Rule2 {
    fn apply(&self, part: &Part2) -> Vec<Part2> {
        let mut result = Vec::new();
        let mut default = part.clone();
        default.state = self.default.to_string();
        for branch in &self.branches {
            if branch.target == "R" {
                continue;
            }
            let mut curr = part.clone();
            match &branch.parameter {
                b'x' => {
                    if let Some(r) = new_range(&curr.x, branch.constant, branch.greater) {
                        curr.x = r;
                    }
                    if let Some(r) = new_range(&default.x, branch.constant, !branch.greater) {
                        default.x = r;
                    }
                },
                b'm' => {
                    if let Some(r) = new_range(&curr.m, branch.constant, branch.greater) {
                        curr.m = r;
                    }
                    if let Some(r) = new_range(&default.m, branch.constant, !branch.greater) {
                        default.m = r;
                    }
                },
                b'a' => {
                    if let Some(r) = new_range(&curr.a, branch.constant, branch.greater) {
                        curr.a = r;
                    }
                    if let Some(r) = new_range(&default.a, branch.constant, !branch.greater) {
                        default.a = r;
                    }
                },
                b's' => {
                    if let Some(r) = new_range(&curr.s, branch.constant, branch.greater) {
                        curr.s = r;
                    }
                    if let Some(r) = new_range(&default.s, branch.constant, !branch.greater) {
                        default.s = r;
                    }
                },
                _ => panic!("how"),
            };
            curr.state = branch.target.to_string();
            result.push(curr);
        }
        result.push(default);
        result
    }
}

pub fn exec_day19_part2(input: &str) -> String {
    let regex_rule_outer = Regex::new(r"([a-zA-Z]+)\{(.*),([a-zA-Z]+)\}").unwrap();
    let regex_branch = Regex::new(r"([xmas])+([<>])(\d+):([a-zA-Z]+)").unwrap();
    let input = input.split("\n\n").collect_vec();
    let mut rules  = HashMap::new();
    for rule in input[0].lines() {
        let cap = regex_rule_outer.captures(rule).unwrap();
        let cap2 = regex_branch.captures_iter(cap.get(2).unwrap().as_str());
        let branches = cap2.map(|m| {
            Branch {
                parameter: m.get(1).unwrap().as_str().as_bytes()[0],
                greater: m.get(2).unwrap().as_str() == ">",
                constant: m.get(3).unwrap().as_str().parse().unwrap(),
                target: m.get(4).unwrap().as_str().to_string(),
            }
        }).collect_vec();
        rules.insert(cap.get(1).unwrap().as_str().to_string(), Rule2 { branches, default: cap.get(3).unwrap().as_str().to_string() });
    }
    let mut processing = vec![Part2 {
        x: 1..4001,
        m: 1..4001,
        a: 1..4001,
        s: 1..4001,
        state: "in".to_string(),
    }];

    let mut accapted = Vec::new();

    while let Some(curr) = processing.pop() {
        let next_parts = rules[&curr.state].apply(&curr);
        for part in next_parts {
            if part.state == "A" {
                accapted.push(part);
            } else if part.state == "R" {
                continue;
            } else {
                processing.push(part);
            }
        }
    }
    println!("{:?}", accapted);

    accapted.iter().fold(0, |acc,p| acc + p.x.clone().count() * p.m.clone().count() * p.a.clone().count() * p.s.clone().count()).to_string()
}
