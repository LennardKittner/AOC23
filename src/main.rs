use std::env;

use aoc23::{run, generate};

fn main() {
    let args :Vec<String> = env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("--run") => {
            match args.get(2).map(|d| d.parse()) {
                Some(Ok(d @ 1..=25)) => run(d),
                _ => println!("please enter the day you want to run as an int from 1 to 25"),
            };
        },
        Some("--generate") => {
            match args.get(2).map(|d| d.parse()) {
                Some(Ok(d @ 1..=25)) => generate(d),
                _ => println!("please enter the day you want to generate a template for as an int from 1 to 25"),
            };
        },
        Some(_) => println!("Unknown command try --run or --generate followed by the day"),
        None => println!("try --run or --generate followed by the day"),
    }
}
