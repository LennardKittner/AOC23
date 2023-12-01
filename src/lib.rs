use std::{fs, i32, time::Instant};

use day1::{exec_day1_part2, exec_day1_part1};

mod day1;

fn exec(fun: impl Fn(&str) -> String, input: &str) {
    let start_time = Instant::now();
    let result = fun(input);
    let end_time = Instant::now();
    println!("result: {result} time: {:.3}ms", (end_time - start_time).as_secs_f64() * 1000.0);
}

pub fn run(day: i32) {
    let input = match fs::read_to_string(format!("./input/day{}.txt", day)) {
        Ok(s) => s,
        Err(_) => return,
    };
    match day {
        t if t < 0 => (),
        1 => {
            exec(exec_day1_part1, &input);
            exec(exec_day1_part2, &input);
        },
        _ => (),
    }
}