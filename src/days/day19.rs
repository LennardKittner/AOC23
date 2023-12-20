use regex::Regex;

pub fn exec_day19_part1(input: &str) -> String {
    let regex = Regex::new(r"([a-zA-Z]+)\{(?:([xmas])+([<>])(\d+):([a-zA-Z]+),)+([a-zA-Z]+)\}");
}

pub fn exec_day19_part2(input: &str) -> String {
    1.to_string()
}
