use std::collections::{HashMap, HashSet};
use num_integer::lcm;

pub fn exec_day8_part1(input: &str) -> String {
    let mut lines = input.lines();
    let directions = lines.next().unwrap().as_bytes();
    let mut map = HashMap::new();
    for line in lines.skip(1) {
        let parts: Vec<String> = line.replace("= ", "").replace([',', '(', ')'], "").split(' ').map(|s|s.to_string()).collect();
        map.insert(parts[0].clone(), (parts[1].clone(), parts[2].clone()));
    }
    let mut current = "AAA";
    let mut i = 0;
    while current != "ZZZ" {
        if directions[i % directions.len()] == 76 {
            current = map[current].0.as_str()
        } else {
            current = map[current].1.as_str()
        }
        i += 1;
    }
    i.to_string()
}

#[derive(Debug)]
struct LoopData {
    target: String,
    index: usize,
    length: usize,
}

pub fn exec_day8_part2(input: &str) -> String {
    let mut lines = input.lines();
    let directions = lines.next().unwrap().as_bytes();
    let mut map = HashMap::new();
    let mut starts = HashSet::new();
    let mut ends = HashSet::new();
    for line in lines.skip(1) {
        let parts: Vec<String> = line.replace("= ", "").replace([',', '(', ')'], "").split(' ').map(|s|s.to_string()).collect();
        map.insert(parts[0].clone(), (parts[1].clone(), parts[2].clone()));
        if parts[0].contains('A') {
            starts.insert(parts[0].clone());
        } else if parts[0].contains('Z') {
            ends.insert(parts[0].clone());
        }
    }
    let mut current: Vec<&str> = starts.iter().map(|s| s.as_str()).collect();
    let mut loops: HashMap<&str, LoopData> = HashMap::new();
    let mut i = 0;
    let mut loops_found = 0;
    while loops_found < starts.len() {
        for curr in current.iter_mut() {
            if directions[i % directions.len()] == 76 {
                *curr = map[*curr].0.as_str();
            } else {
                *curr = map[*curr].1.as_str();
            }
        }
        for j in (0..current.len()).rev() {
            if current[j].contains('Z') {
                if !loops.contains_key(current[j]) {
                    loops.insert(current[j], LoopData { target: current[j].to_string(), index: i, length: 0});
                } else {
                    let data = loops.get_mut(current[j]).unwrap();
                    if data.target == *current[j] && i % directions.len() == data.index % directions.len() {
                        data.length = i - data.index;
                        loops_found += 1;
                        current.remove(j);
                    }
                }
            }
        }
        i += 1;
    }
    loops.values().fold(1, |a, d| lcm(a, d.length)).to_string()
}
