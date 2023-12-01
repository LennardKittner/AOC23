use core::num;
use std::i32;

pub fn exec_day1_part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut result: u64 = 0;
    for line in lines {
        let mut nums = Vec::new();
        for c in line.bytes() {
            if c >= 48 && c <= 57 {
                nums.push(c-48);
            }
        }
        result += (nums.first().unwrap_or(&0) * 10 + nums.last().unwrap_or(&0)) as u64;
    }
    return result.to_string();
}

pub fn exec_day1_part2(input: &str) -> String {
    let input = input.replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");
    let lines: Vec<&str> = input.lines().collect();
    let mut result: u64 = 0;
    for line in lines {
        let mut nums = Vec::new();
        for c in line.bytes() {
            if c >= 48 && c <= 57 {
                nums.push(c-48);
            }
        }
        result += (nums.first().unwrap_or(&0) * 10 + nums.last().unwrap_or(&0)) as u64;
    }
    return result.to_string();
}