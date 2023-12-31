use std::{fs, i32};

use crate::exec;

mod day20;
mod day14;
mod day9;
mod day10;
mod day24;
mod day11;
mod day25;
mod day21;
mod day15;
mod day8;
mod day3;
mod day7;
mod day6;
mod day2;
mod day18;
mod day5;
mod day1;
mod day19;
mod day4;
mod day12;
mod day22;
mod day16;
mod day17;
mod day13;
mod day23;


pub fn run(day: i32) {
    let input = match fs::read_to_string(format!("./input/day{}.txt", day)) {
        Ok(s) => s,
        Err(_) => return,
    };
    match day {
        t if t < 0 => (),
        20 => {
            exec(day20::exec_day20_part1, &input);
            exec(day20::exec_day20_part2, &input);
        },
        14 => {
            exec(day14::exec_day14_part1, &input);
            exec(day14::exec_day14_part2, &input);
        },
        9 => {
            exec(day9::exec_day9_part1, &input);
            exec(day9::exec_day9_part2, &input);
        },
        10 => {
            exec(day10::exec_day10_part1, &input);
            exec(day10::exec_day10_part2, &input);
        },
        24 => {
            exec(day24::exec_day24_part1, &input);
            exec(day24::exec_day24_part2, &input);
        },
        11 => {
            exec(day11::exec_day11_part1, &input);
            exec(day11::exec_day11_part2, &input);
        },
        25 => {
            exec(day25::exec_day25_part1, &input);
            exec(day25::exec_day25_part2, &input);
        },
        21 => {
            exec(day21::exec_day21_part1, &input);
            exec(day21::exec_day21_part2, &input);
        },
        15 => {
            exec(day15::exec_day15_part1, &input);
            exec(day15::exec_day15_part2, &input);
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
        18 => {
            exec(day18::exec_day18_part1, &input);
            exec(day18::exec_day18_part2, &input);
        },
        5 => {
            exec(day5::exec_day5_part1, &input);
            exec(day5::exec_day5_part2, &input);
        },
        1 => {
            exec(day1::exec_day1_part1, &input);
            exec(day1::exec_day1_part2, &input);
        },
        19 => {
            exec(day19::exec_day19_part1, &input);
            exec(day19::exec_day19_part2, &input);
        },
        4 => {
            exec(day4::exec_day4_part1, &input);
            exec(day4::exec_day4_part2, &input);
        },
        12 => {
            exec(day12::exec_day12_part1, &input);
            exec(day12::exec_day12_part2, &input);
        },
        22 => {
            exec(day22::exec_day22_part1, &input);
            exec(day22::exec_day22_part2, &input);
        },
        16 => {
            exec(day16::exec_day16_part1, &input);
            exec(day16::exec_day16_part2, &input);
        },
        17 => {
            exec(day17::exec_day17_part1, &input);
            exec(day17::exec_day17_part2, &input);
        },
        13 => {
            exec(day13::exec_day13_part1, &input);
            exec(day13::exec_day13_part2, &input);
        },
        23 => {
            exec(day23::exec_day23_part1, &input);
            exec(day23::exec_day23_part2, &input);
        },
        _ => (),
    }
}
