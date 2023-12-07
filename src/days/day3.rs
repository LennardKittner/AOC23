use regex::Regex;


fn get_num(string: &str, index: i32) -> i32 {
    let num_regex = Regex::new(r"\d+").unwrap();
    let mut result = 0;
    for num in num_regex.find_iter(string) {
        let start = num.start() as i32;
        let end = num.end() as i32;
        if index+1 >= start && index <= end {
            result += num.as_str().parse::<i32>().unwrap();
        }
    }
     result
}

fn get_num_mul(string: &str, index: i32) -> (i32, i32) {
    let num_regex = Regex::new(r"\d+").unwrap();
    let mut result = 1;
    let mut count = 0;
    for num in num_regex.find_iter(string) {
        let start = num.start() as i32;
        let end = num.end() as i32;
        if index+1 >= start && index <= end {
            count += 1;
            result *= num.as_str().parse::<i32>().unwrap();
        }
    }
    (result, count)
}

pub fn exec_day3_part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut result = 0;
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.as_bytes().iter().enumerate() {
            if (48..=57).contains(c) && c != &46 {
                result += get_num(lines[i], j as i32);
                if i > 0 {  result += get_num(lines[i-1], j as i32); }
                if i + 1 < lines.len() { result += get_num(lines[i+1], j as i32); }
            }
        }
    }
    result.to_string()
}

pub fn exec_day3_part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut result = 0;
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.as_bytes().iter().enumerate() {
            if c == &42 {
                let cur = get_num_mul(lines[i], j as i32);
                let down = if i > 0 { get_num_mul(lines[i-1], j as i32) } else { (1, 0) };
                let up = if i + 1 < lines.len() {get_num_mul(lines[i+1], j as i32) } else { (1, 0) };
                if cur.1 + down.1 + up.1 == 2 {
                    result += cur.0 * down.0 * up.0;
                }
            }
        }
    }
    result.to_string()
}