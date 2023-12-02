use std::{env, num::ParseIntError};

use aoc23::run;

fn main() {
    let args :Vec<Result<i32, ParseIntError>> = env::args().map(|s| s.parse::<i32>()).collect();
    let task = match args.get(1) {
        Some(t) => t.clone().unwrap_or(-1),
        None => -1,
    };
    if task < 0 {
        println!("please enter the day you want to run as an int from 1 to 24");
        return;
    }
    run(task);
}
