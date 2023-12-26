use itertools::Itertools;

pub fn exec_day18_part1(input: &str) -> String {
    let moves = input.lines().map(|line| {
        let values = line.split(' ').collect_vec();
        (values[0].as_bytes()[0], values[1].parse::<i64>().unwrap())
    }).collect_vec();
    let mut points = Vec::new();
    points.push((0i64, 0i64));
    let mut curr = (0i64, 0i64);
    let mut bonus = 0;
    for mov in &moves {
        curr = match mov.0 {
            b'U' => (curr.0, curr.1 - mov.1),
            b'D' => {bonus += mov.1; (curr.0, curr.1 + mov.1)},
            b'L' => (curr.0 - mov.1, curr.1),
            b'R' => {bonus += mov.1; (curr.0 + mov.1, curr.1)},
            _ => panic!("HOW"),
        };
        points.push(curr);
    }
    day18(&points, &bonus)
}

fn day18(points: &Vec<(i64, i64)>, bonus: &i64) -> String {
    let mut result: i64 = 0;
    let mut j = &points.len() - 1;
    for (i, point) in points.iter().enumerate() {
        result += (point.0 + points[j].0) * (points[j].1 - point.1);
        j = i;
    }
    (result.abs() / 2 + bonus + 1).to_string()
}

pub fn exec_day18_part2(input: &str) -> String {
    let moves = input.lines().map(|line| {
        let values = line.split(' ').collect_vec();
        let num = i64::from_str_radix(&values[2][2..7], 16).unwrap();
        let dir = i64::from_str_radix(&values[2][7..8], 16).unwrap();
        (dir, num)
    }).collect_vec();
    let mut points = Vec::new();
    points.push((0i64, 0i64));
    let mut curr = (0i64, 0i64);
    let mut bonus = 0;
    for mov in &moves {
        curr = match mov.0 {
            3 => (curr.0, curr.1 - mov.1),
            1 => {bonus += mov.1; (curr.0, curr.1 + mov.1)},
            2 => (curr.0 - mov.1, curr.1),
            0 => {bonus += mov.1; (curr.0 + mov.1, curr.1)},
            _ => panic!("HOW"),
        };
        points.push(curr);
    }
    day18(&points, &bonus)
}
