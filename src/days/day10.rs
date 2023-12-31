use std::collections::HashSet;
use itertools::Itertools;
use crate::days::day10::Direction::{Down, Left, Right, Up};
use crate::days::day10::Pipe::{DownLeft, DownRight, Ground, LeftRight, Start, UpDown, UpLeft, UpRight};

#[derive(Debug)]
enum Pipe {
    UpDown,
    LeftRight,
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
    Ground,
    Start,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn get_pip(curr: &char) -> Pipe {
    match curr {
        '|' => UpDown,
        '-' => LeftRight,
        'L' => UpRight,
        'J' => UpLeft,
        '7' => DownLeft,
        'F' => DownRight,
        '.' => Ground,
        'S' => Start,
        _ => Ground
    }
}

pub fn exec_day10_part1(input: &str) -> String {
    let grid = input.lines().map(|l| l.as_bytes()).collect_vec();
    let start = find_start(&grid);
    let mut curr_cord = start;
    let mut curr = 'L'; //TODO: calc
    let mut from = Down; //TODO: calc
    let mut len = 0;
    while curr != 'S' {
        (curr_cord, from) = get_next(&curr_cord, &curr, &from);
        curr = grid[curr_cord.1][curr_cord.0] as char;
        len += 1;
    }
    (len/2).to_string()
}

fn find_start(grid: &Vec<&[u8]>) -> (usize, usize) {
    let mut start = (0, 0);
    for y in 0..grid.len() {
        for x in 0..grid.first().unwrap().len() {
            if grid[y][x] == b'S' {
                start = (x, y);
                break;
            }
        }
    }
    start
}

fn get_next(curr_cord: &(usize, usize), curr: &char, from: &Direction) -> ((usize, usize), Direction) {
    match (get_pip(curr), &from) {
        (UpDown, Up) => ((curr_cord.0, curr_cord.1 - 1), Up),
        (UpDown, Down) => ((curr_cord.0, curr_cord.1 + 1), Down),
        (LeftRight, Left) => ((curr_cord.0 - 1, curr_cord.1), Left),
        (LeftRight, Right) => ((curr_cord.0 + 1, curr_cord.1), Right),
        (UpRight, Down) => ((curr_cord.0 + 1, curr_cord.1), Right),
        (UpRight, Left) => ((curr_cord.0, curr_cord.1 - 1), Up),
        (UpLeft, Down) => ((curr_cord.0 - 1, curr_cord.1), Left),
        (UpLeft, Right) => ((curr_cord.0, curr_cord.1 - 1), Up),
        (DownLeft, Up) => ((curr_cord.0 - 1, curr_cord.1), Left),
        (DownLeft, Right) => ((curr_cord.0, curr_cord.1 + 1), Down),
        (DownRight, Up) => ((curr_cord.0 + 1, curr_cord.1), Right),
        (DownRight, Left) => ((curr_cord.0, curr_cord.1 + 1), Down),
        (a, b) => panic!("{:?} {:?}", a, b)
    }
}

fn is_inner(dim_x: usize, cycle: &HashSet<((usize, usize), Direction, char)>, cycle2: &HashSet<(usize, usize)>, pos: (usize, usize)) -> bool {
    if cycle2.contains(&pos) {
        return false;
    }
    for x in pos.0..dim_x {
        if cycle.contains(&((x, pos.1), Down, '|')) {
            return true;
        }
        if cycle.contains(&((x, pos.1), Up, '|')) {
            return false;
        }
        if cycle.contains(&((x, pos.1), Down, 'L')) {
            return true;
        }
        if cycle.contains(&((x, pos.1), Left, 'F')) {
            return true;
        }
        if cycle2.contains(&(x, pos.1)) {
            return false;
        }
    }
    false
}

pub fn exec_day10_part2(input: &str) -> String {
    let grid = input.lines().map(|l| l.as_bytes()).collect_vec();
    let start = find_start(&grid);
    let mut cycle = HashSet::new();
    let mut cycle2 = HashSet::new();
    let mut curr_cord = start;
    let mut curr = 'L'; //TODO: calc
    let mut from = Left; //TODO: calc
    cycle.insert((curr_cord, from, curr));
    while curr != 'S' {
        (curr_cord, from) = get_next(&curr_cord, &curr, &from);
        curr = grid[curr_cord.1][curr_cord.0] as char;
        cycle.insert((curr_cord, from, curr));
        cycle2.insert(curr_cord);
    }

    let mut result = 0;
    for y in 0..grid.len() {
        for x in 0..grid.first().unwrap().len() {
            if is_inner(grid.first().unwrap().len(), &cycle, &cycle2, (x, y)) {
                result += 1;
            }
        }
    }
    result.to_string()
}
