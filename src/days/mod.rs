use std::{fs, i32};

use crate::exec;

mod day9;
mod day10;
mod day11;
mod day8;
mod day3;
mod day7;
mod day6;
mod day2;
mod day5;
mod day1;
mod day4;
mod day12;


pub fn run(day: i32) {
    let input = match fs::read_to_string(format!("./input/day{}.txt", day)) {
        Ok(s) => s,
        Err(_) => return,
    };
    match day {
        t if t < 0 => (),
        9 => {
            exec(day9::exec_day9_part1, &input);
            exec(day9::exec_day9_part2, &input);
        },
        10 => {
            exec(day10::exec_day10_part1, &input);
            exec(day10::exec_day10_part2, &input);
        },
        11 => {
            exec(day11::exec_day11_part1, &input);
            exec(day11::exec_day11_part2, &input);
        },
        8 => {
            exec(day8::exec_day8_part1, &input);
            exec(day8::exec_day8_part2, &input);
        },
        3 => {
            exec(day3::exec_day3_part1, &input);
            exec(day3::exec_day3_part2, &input);
        },
        7 => {
            exec(day7::exec_day7_part1, &input);
            exec(day7::exec_day7_part2, &input);
        },
        6 => {
            exec(day6::exec_day6_part1, &input);
            exec(day6::exec_day6_part2, &input);
        },
        2 => {
            exec(day2::exec_day2_part1, &input);
            exec(day2::exec_day2_part2, &input);
        },
        5 => {
            exec(day5::exec_day5_part1, &input);
            exec(day5::exec_day5_part2, &input);
        },
        1 => {
            exec(day1::exec_day1_part1, &input);
            exec(day1::exec_day1_part2, &input);
        },
        4 => {
            exec(day4::exec_day4_part1, &input);
            exec(day4::exec_day4_part2, &input);
        },
        12 => {
            exec(day12::exec_day12_part1, &input);
            exec(day12::exec_day12_part2, &input);
        },
        _ => (),
    }
}
