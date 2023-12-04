use std::{fs::{self, File}, i32, time::Instant, io::Write};

use indoc::formatdoc;

mod days;

fn exec(fun: impl Fn(&str) -> String, input: &str) {
    let start_time = Instant::now();
    let result = fun(input);
    let end_time = Instant::now();
    println!("result: {result} time: {:.3}ms", (end_time - start_time).as_secs_f64() * 1000.0);
}

fn generate_mod_string(days: &Vec<i32>) -> String {
    let mut result: String = String::new();
    for i in days {
        result += format!("mod day{i};\n").as_str();
    }
    return result;
}

fn generate_math_branches(days: &Vec<i32>) -> String {
    let mut result: String = String::new();
    for i in days {
        result += format!(r#"
        {i} => {{
            exec(day{i}::exec_day{i}_part1, &input);
            exec(day{i}::exec_day{i}_part2, &input);
        }},"#).as_str();
    }
    return result;
}

pub fn generate(day: i32) {
    let _ = File::create(format!("./input/day{day}.txt")).expect("Failed to generate input");
    let src_file_path = format!("./src/days/day{day}.rs");
    let days_dir_path = "./src/days";
    let mod_path = "./src/days/mod.rs";
    let days: Vec<i32> = fs::read_dir(days_dir_path).unwrap().filter_map(|entry| 
            entry.unwrap().file_name().to_string_lossy().chars()
                .filter(|c| 
                    c.is_numeric()
                ).collect::<String>().parse::<i32>().ok()
        ).collect();
    if days.contains(&day) {
        println!("skipping code generation of {src_file_path} already exists");
    } else {
        let mut new_day = File::create(&src_file_path).expect("Failed to generate source.");
        let _ = new_day.write_all(formatdoc!{
            r#"
            pub fn exec_day{day}_part1(input: &str) -> String {{
                todo!("{{input}}")
            }}

            pub fn exec_day{day}_part2(input: &str) -> String {{
                todo!("{{input}}")
            }}
            "#
        }.as_bytes()).expect(format!("Failed to write {src_file_path}.").as_str());
    }
    let mut new_mod = File::create(mod_path).expect(format!("Failed to generate {mod_path}.").as_str());
    let _ = new_mod.write_all( formatdoc!{
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
        "#, generate_mod_string(&days), generate_math_branches(&days)
    }.as_bytes()).expect(format!("Failed to write {mod_path}.").as_str());
}

pub fn run(day: i32) {
    days::run(day);
}