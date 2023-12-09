use std::{fs, i32};
use std::collections::HashMap;
use std::time::Duration;

use crate::{exec, get_days, time};

mod day3;
mod day4;
mod day1;
mod day7;
mod day2;
mod day8;
mod day5;
mod day6;

pub fn benchmark(days: &[i32]) -> HashMap<i32, (Duration, Duration)> {
    let mut result = HashMap::new();
    let days = if days.is_empty() { get_days() } else { days.to_vec() };
    for day in days {
        let input = match fs::read_to_string(format!("./input/day{}.txt", day)) {
            Ok(s) => s,
            Err(_) => return HashMap::new(),
        };
        result.insert(day ,match day {
            3 => {
                (time(day3::exec_day3_part1, &input), time(day3::exec_day3_part2, &input))
            },
            4 => {
                (time(day4::exec_day4_part1, &input), time(day4::exec_day4_part2, &input))
            },
            _ => (Duration::new(0, 0), Duration::new(0, 0)),
        });
    }
    result
}


pub fn run(day: i32) {
    let input = match fs::read_to_string(format!("./input/day{}.txt", day)) {
        Ok(s) => s,
        Err(_) => return,
    };
    match day {
        t if t < 0 => (),
        3 => {
            exec(day3::exec_day3_part1, &input);
            exec(day3::exec_day3_part2, &input);
        },
        4 => {
            exec(day4::exec_day4_part1, &input);
            exec(day4::exec_day4_part2, &input);
        },
        1 => {
            exec(day1::exec_day1_part1, &input);
            exec(day1::exec_day1_part2, &input);
        },
        7 => {
            exec(day7::exec_day7_part1, &input);
            exec(day7::exec_day7_part2, &input);
        },
        2 => {
            exec(day2::exec_day2_part1, &input);
            exec(day2::exec_day2_part2, &input);
        },
        8 => {
            exec(day8::exec_day8_part1, &input);
            exec(day8::exec_day8_part2, &input);
        },
        5 => {
            exec(day5::exec_day5_part1, &input);
            exec(day5::exec_day5_part2, &input);
        },
        6 => {
            exec(day6::exec_day6_part1, &input);
            exec(day6::exec_day6_part2, &input);
        },
        _ => (),
    }
}
