use regex::Regex;

const RED_LIMIT: i32 = 12;
const GREEN_LIMIT: i32 = 13;
const BLUE_LIMIT: i32 = 14;

pub fn exec_day2_part1(input: &str) -> String {
    let id_regex = Regex::new(r"Game (\d+)").unwrap();
    let colors_regex = Regex::new(r"(?:(?P<red>\d+) red(?:\, )?|(?P<blue>\d+) blue(?:\, )?|(?P<green>\d+) green(?:\, )?)+").unwrap();
    let mut result = 0;
    for line in input.lines() {
        let id = id_regex.captures(line).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
        let rounds = line.split(": ").collect::<Vec<&str>>()[1];
        let mut valid = true;
        for round in rounds.split("; ") {
            let colors = colors_regex.captures(round).unwrap();
            let red = match colors.name("red") {
                Some(num) => num.as_str().parse::<i32>().unwrap(),
                None => 0,
            };
            let blue = match colors.name("blue") {
                Some(num) => num.as_str().parse::<i32>().unwrap(),
                None => 0,
            };
            let green = match colors.name("green") {
                Some(num) => num.as_str().parse::<i32>().unwrap(),
                None => 0,
            };
            if red > RED_LIMIT || green > GREEN_LIMIT || blue > BLUE_LIMIT {
                valid = false
            }
        }
        if valid {
            result += id;
        }
    }
    return format!("{result}");
}

pub fn exec_day2_part2(input: &str) -> String {
    let colors_regex = Regex::new(r"(?:(?P<red>\d+) red(?:\, )?|(?P<blue>\d+) blue(?:\, )?|(?P<green>\d+) green(?:\, )?)+").unwrap();
    let mut result = 0;
    for line in input.lines() {
        let rounds = line.split(": ").collect::<Vec<&str>>()[1];
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for round in rounds.split("; ") {
            let colors = colors_regex.captures(round).unwrap();
            let red = match colors.name("red") {
                Some(num) => num.as_str().parse::<i32>().unwrap(),
                None => 0,
            };
            let blue = match colors.name("blue") {
                Some(num) => num.as_str().parse::<i32>().unwrap(),
                None => 0,
            };
            let green = match colors.name("green") {
                Some(num) => num.as_str().parse::<i32>().unwrap(),
                None => 0,
            };
            max_red = std::cmp::max(max_red, red);
            max_green = std::cmp::max(max_green, green);
            max_blue = std::cmp::max(max_blue, blue);
        }
        result += max_red * max_green * max_blue;
    }
    return format!("{result}");
}