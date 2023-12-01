use std::{fs, i32};

use day1::{exec_day1_part2, exec_day1_part1};

mod day1;

pub fn run(day: i32) {
    let input = match fs::read_to_string("./input/day1.txt") {
        Ok(s) => s,
        Err(_) => return,
    };
    match day {
        t if t < 0 => (),
        1 => {
            exec_day1_part1(&input);
            exec_day1_part2(&input);
        },
        _ => (),
    }
}