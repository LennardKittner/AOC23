use std::collections::HashMap;
use std::ops::Range;
use itertools::Itertools;
use regex::Regex;
use crate::days::day19::Comparison::{Greater, GreaterOrEqual, Less, LessOrEqual};

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

#[derive(Debug)]
enum Comparison {
    Greater,
    Less,
    GreaterOrEqual,
    LessOrEqual,
}

impl Comparison {
    fn neg(&self) -> Self {
       match self {
           Greater => LessOrEqual,
           Less => GreaterOrEqual,
           GreaterOrEqual => Less,
           LessOrEqual => Greater
       }
    }
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
struct Branch2 {
    parameter: u8,
    comparison: Comparison,
    constant: i64,
    target: String,
}

#[derive(Debug)]
struct Rule2 {
    branches: Vec<Branch2>,
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

fn new_range(range: &Range<i64>, val: i64, comparison: &Comparison) -> Option<Range<i64>> {
    match comparison {
        Greater if range.end > val+1 => Some(range.start.max(val+1)..range.end),
        Less if range.start < val => Some(range.start..range.end.min(val)),
        GreaterOrEqual if range.end > val => Some(range.start.max(val)..range.end),
        LessOrEqual if range.start <= val => Some(range.start..range.end.min(val+1)),
        _ => None
    }
}

impl Rule2 {
    fn apply(&self, part: &Part2) -> Vec<Part2> {
        let mut result = Vec::new();
        let mut curr_false = part.clone();
        for branch in &self.branches {
            let mut curr_true = curr_false.clone();
            curr_false = curr_false.clone();
            match &branch.parameter {
                b'x' => {
                    if let Some(r) = new_range(&curr_true.x, branch.constant, &branch.comparison) {
                        curr_true.x = r;
                    }
                    if let Some(r) = new_range(&curr_false.x, branch.constant, &branch.comparison.neg()) {
                        curr_false.x = r;
                    }
                },
                b'm' => {
                    if let Some(r) = new_range(&curr_true.m, branch.constant, &branch.comparison) {
                        curr_true.m = r;
                    }
                    if let Some(r) = new_range(&curr_false.m, branch.constant, &branch.comparison.neg()) {
                        curr_false.m = r;
                    }
                },
                b'a' => {
                    if let Some(r) = new_range(&curr_true.a, branch.constant, &branch.comparison) {
                        curr_true.a = r;
                    }
                    if let Some(r) = new_range(&curr_false.a, branch.constant, &branch.comparison.neg()) {
                        curr_false.a = r;
                    }
                },
                b's' => {
                    if let Some(r) = new_range(&curr_true.s, branch.constant, &branch.comparison) {
                        curr_true.s = r;
                    }
                    if let Some(r) = new_range(&curr_false.s, branch.constant, &branch.comparison.neg()) {
                        curr_false.s = r;
                    }
                },
                _ => panic!("how"),
            };
            if branch.target != "R" {
                curr_true.state = branch.target.to_string();
                result.push(curr_true);
            }
        }
        curr_false.state = self.default.clone();
        result.push(curr_false);
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
            Branch2 {
                parameter: m.get(1).unwrap().as_str().as_bytes()[0],
                comparison: if m.get(2).unwrap().as_str() == ">" { Greater } else { Less } ,
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

    let mut accepted = Vec::new();
    while let Some(curr) = processing.pop() {
        let next_parts = rules[&curr.state].apply(&curr);
        for part in next_parts {
            if part.state == "A" {
                accepted.push(part);
            } else if part.state == "R" {
                continue;
            } else {
                processing.push(part);
            }
        }
    }
    accepted.iter().fold(0, |acc, p| acc + p.x.clone().count() * p.m.clone().count() * p.a.clone().count() * p.s.clone().count()).to_string()
}