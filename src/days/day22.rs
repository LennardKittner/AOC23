use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Brick {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
    z: RangeInclusive<i64>,
    parents: HashSet<usize>,
    children: HashSet<usize>,
}

pub fn exec_day22_part1(input: &str) -> String {
    let mut bricks = parse(input);
    simulate_falling(&mut bricks);
    let mut result = 0;
    for brick in &bricks {
        let mut safe = true;
        for parent in &brick.parents {
            if bricks[*parent].children.len() == 1usize {
                safe = false;
                break;
            }
        }
        if safe {
            result += 1;
        }
    }
    result.to_string()
}

fn parse(input: &str) -> Vec<Brick> {
    let mut bricks = input.lines().map(|line| {
        let values = line.split('~').map(|half| half.split(',').map(|n| n.trim().parse::<i64>().unwrap()).collect_vec()).collect_vec();
        Brick {
            x: values[0][0]..=values[1][0],
            y: values[0][1]..=values[1][1],
            z: values[0][2]..=values[1][2],
            parents: HashSet::new(),
            children: HashSet::new(),
        }
    }).collect_vec();
    bricks.sort_by(|b1, b2| {
        b1.z.start().cmp(b2.z.start())
    });
    bricks
}

pub fn exec_day22_part2(input: &str) -> String {
    let mut bricks = parse(input);
    simulate_falling(&mut bricks);

    let mut result = 0;
    for i in 0..bricks.len() {
        let mut working_bricks = bricks.clone();
        working_bricks[i].children.clear();
        result += delete_recursive(&mut working_bricks, i) - 1;
    }
    result.to_string()
}

fn delete_recursive(bricks: &mut Vec<Brick>, curr: usize) -> u64 {
    if !bricks[curr].children.is_empty() {
        return 0;
    }
    let mut result = 1;
    for brick in &bricks[curr].parents.clone() {
        bricks[*brick].children.remove(&curr);
        result += delete_recursive(bricks, *brick);
    }
    result
}

fn simulate_falling(bricks: &mut Vec<Brick>) {
    let mut occupied: HashMap<(i64, i64, i64), usize> = HashMap::new();
    for i in 0..bricks.len() {
        let mut new_z: i64 = 1;
        for z in (1i64..*bricks[i].z.start()).rev() {
            for x in bricks[i].x.clone() {
                for y in bricks[i].y.clone() {
                    if let Some(child) = occupied.get(&(x, y, z)) {
                        bricks[*child].parents.insert(i);
                        bricks[i].children.insert(*child);
                        new_z = z + 1;
                    }
                }
            }
            if new_z != 1 {
                break;
            }
        }
        for x in bricks[i].x.clone() {
            for y in bricks[i].y.clone() {
                occupied.insert((x, y, new_z + bricks[i].z.try_len().unwrap() as i64 - 1), i);
            }
        }
    }
}
