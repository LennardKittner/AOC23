use std::{collections::HashMap, cmp::Ordering};

#[derive(Debug)]
struct Entry {
    hand: Vec<i32>,
    bid: u64
}

fn value(entry: &Entry) -> i32 {
    let mut encounters: HashMap<i32, i32>  = HashMap::new();
    for card in &entry.hand {
        if encounters.contains_key(card) {
            *encounters.get_mut(card).unwrap() += 1;
        } else {
            encounters.insert(*card, 1);
        }
    }
    let max = encounters.values().max().unwrap();
    if *max == 5 {
        return 7;
    }
    if *max == 4 {
        return 6;
    }
    let three = *max == 3;
    let mut pairs = 0;
    for (_, v) in encounters {
        if v == 2 {
            pairs += 1;
        }
    }
    if three && pairs == 1 {
        return 5;
    }
    if three {
        return 4;
    }
    if pairs == 2 {
        return 3;
    }
    if pairs == 1 {
        return 2;
    }
    1
}

fn compare(entry1: &Entry, entry2: &Entry, value: fn(&Entry) -> i32) -> Ordering {
    let val1 = value(entry1);
    let val2 = value(entry2);
    match val1.cmp(&val2) {
        Ordering::Equal => (),
        o => return o,
    }
    let mut entry1_won = true;
    for i in 0..entry1.hand.len() {
        if entry1.hand[i] < entry2.hand[i] {
            entry1_won = false;
            break;
        }
        if entry1.hand[i] > entry2.hand[i] {
            break;
        }
    }
    if entry1_won { Ordering::Greater } else { Ordering::Less }
}

pub fn exec_day7_part1(input: &str) -> String {
    let mut map: HashMap<char, i32> = HashMap::new();
    map.insert('A', 14);
    map.insert('K', 13);
    map.insert('Q', 12);
    map.insert('J', 11);
    map.insert('T', 10);
    map.insert('9', 9);
    map.insert('8', 8);
    map.insert('7', 7);
    map.insert('6', 6);
    map.insert('5', 5);
    map.insert('4', 4);
    map.insert('3', 3);
    map.insert('2', 2);
    let mut entries = parse(input, map);
    entries.sort_by(|e1, e2| compare(e1, e2, value));
    entries.iter().enumerate().map(|(i, e)| e.bid * (i+1) as u64).sum::<u64>().to_string()
}

pub fn exec_day7_part2(input: &str) -> String {
    let mut map: HashMap<char, i32> = HashMap::new();
    map.insert('A', 14);
    map.insert('K', 13);
    map.insert('Q', 12);
    map.insert('T', 10);
    map.insert('9', 9);
    map.insert('8', 8);
    map.insert('7', 7);
    map.insert('6', 6);
    map.insert('5', 5);
    map.insert('4', 4);
    map.insert('3', 3);
    map.insert('2', 2);
    map.insert('J', 1);
    let mut entries = parse(input, map);
    entries.sort_by(|e1, e2| compare(e1, e2, value2));
    entries.iter().enumerate().map(|(i, e)| e.bid * (i+1) as u64).sum::<u64>().to_string()
}

fn parse(input: &str, map: HashMap<char, i32>) -> Vec<Entry> {
    let entries: Vec<Entry> = input.lines().map(|l| {
        let parts: Vec<&str> = l.split(' ').collect();
        Entry {
            hand: parts[0].chars().map(|c| map[&c]).collect::<Vec<i32>>(),
            bid: parts[1].parse().unwrap(),
        }
    }).collect();
    entries
}

fn value2(entry: &Entry) -> i32 {
    let mut encounters: HashMap<i32, i32>  = HashMap::new();
    for card in &entry.hand {
        if encounters.contains_key(card) {
            *encounters.get_mut(card).unwrap() += 1;
        } else {
            encounters.insert(*card, 1);
        }
    }
    let jokers = if encounters.contains_key(&1) {
        let tmp = encounters[&1];
        encounters.remove(&1);
        tmp
    } else {0};
    if jokers == 5 {
        return 7;
    }

    let mut max = 0;
    let mut max_key = 0;
    for (k, v) in &encounters {
        if max < *v {
            max = *v;
            max_key = *k;
        }
    }
    *encounters.get_mut(&max_key).unwrap() = max + jokers;
    let max = max + jokers;
    if max == 5 {
        return 7;
    }
    if max == 4 {
        return 6;
    }
    let three = max == 3;
    let mut pairs = 0;
    for v in encounters.values() {
        if *v == 2 {
            pairs += 1;
        }
    }
    if three && pairs == 1 {
        return 5;
    }
    if three {
        return 4;
    }
    if pairs == 2 {
        return 3;
    }
    if pairs == 1 {
        return 2;
    }
    if max == 2 {
        return 2;
    }
    1
}