use std::{fmt::Write, fs::{self, File}, i32, time::Instant, io::Write as IOWrite};
use std::time::Duration;

use indoc::formatdoc;

mod days;

fn exec(fun: impl Fn(&str) -> String, input: &str) {
    let start_time = Instant::now();
    let result = fun(input);
    let end_time = Instant::now();
    println!("result: {result} time: {:?}", (end_time - start_time));
}

#[allow(dead_code)]
fn time(fun: impl Fn(&str) -> String, input: &str) -> Duration {
    let start_time = Instant::now();
    let _ = fun(input);
    let end_time = Instant::now();
    end_time - start_time
}

fn generate_mod_string(days: &Vec<i32>) -> String {
    let mut result: String = String::new();
    for i in days {
        result += format!("mod day{i};\n").as_str();
    }
    result
}

fn generate_match_branches(days: &[i32]) -> String {
    days.iter()
        .fold(String::new(), |mut result, d| { write!(&mut result, r#"
        {d} => {{
            exec(day{d}::exec_day{d}_part1, &input);
            exec(day{d}::exec_day{d}_part2, &input);
        }},"#).unwrap();
            result
        })
}

pub fn generate(day: i32) {
    let _ = File::create(format!("./input/day{day}.txt")).expect("Failed to generate input");
    let src_file_path = format!("./src/days/day{day}.rs");
    let mod_path = "./src/days/mod.rs";
    let mut days = get_days();
    if days.contains(&day) {
        println!("skipping code generation of {src_file_path} already exists");
    } else {
        days.push(day);
        let mut new_day = File::create(&src_file_path).expect("Failed to generate source.");
        new_day.write_all(formatdoc!{
            r#"
            pub fn exec_day{day}_part1(input: &str) -> String {{
                todo!("{{input}}")
            }}

            pub fn exec_day{day}_part2(input: &str) -> String {{
                todo!("{{input}}")
            }}
            "#
        }.as_bytes()).unwrap_or_else(|_| panic!("Failed to write {src_file_path}."));
    }
    let mut new_mod = File::create(mod_path).unwrap_or_else(|_| panic!("Failed to generate {mod_path}."));
    new_mod.write_all( formatdoc!{
        r#"
        use std::{{fs, i32}};

        use crate::exec;

        {}

        pub fn run(day: i32) {{
            let input = match fs::read_to_string(format!("./input/day{{}}.txt", day)) {{
                Ok(s) => s,
                Err(_) => return,
            }};
            match day {{
                t if t < 0 => (),{}
                _ => (),
            }}
        }}
        "#, generate_mod_string(&days), generate_match_branches(&days)
    }.as_bytes()).unwrap_or_else(|_| panic!("Failed to write {mod_path}."));
}

fn get_days() -> Vec<i32> {
    let days_dir_path = "./src/days";
    let days: Vec<i32> = fs::read_dir(days_dir_path).unwrap().filter_map(|entry|
        entry.unwrap().file_name().to_string_lossy().chars()
            .filter(|c|
                c.is_numeric()
            ).collect::<String>().parse::<i32>().ok()
    ).collect();
    days
}

pub fn run(day: i32) {
    days::run(day);
}