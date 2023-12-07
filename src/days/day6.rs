fn ints(string: &str) -> Vec<u64> {
    string.split(' ').filter_map(|s| s.parse::<u64>().ok()).collect()
}

pub fn exec_day6_part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let times = ints(lines[0]);
    let distances = ints(lines[1]);
    let mut result = 1;
    for i in 0..times.len() {
        let mut tmp = 0;
        for hold in 0..=times[i] {
            if hold * (times[i] - hold) > distances[i] {
                tmp += 1;
            }
        }
        result *= tmp;
    }
    result.to_string()
}

pub fn exec_day6_part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let time: u64 = lines[0].chars().filter(|c| c.is_numeric()).collect::<String>().parse().unwrap();
    let distance: u64 = lines[1].chars().filter(|c| c.is_numeric()).collect::<String>().parse().unwrap();
    let mut result = 0;
    for hold in 0..=time {
        if hold * (time - hold) > distance {
            result += 1;
        }
    }
    result.to_string()
}
