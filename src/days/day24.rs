use itertools::Itertools;

struct Hail {
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

// y = mx + b
#[derive(Debug)]
struct Line {
    m: f64,
    b: f64
}

fn hail_to_line(hail: &Hail) -> Line {
    let m = hail.vy / hail.vx;
    Line { m , b: hail.py - m * hail.px }
}

fn find_intersection(line1: &Line, line2: &Line) -> Option<(f64, f64)> {
    if line1.m == line2.m {
        return None;
    }
    let x = (line2.b - line1.b) / (line1.m - line2.m);
    let y = line1.m * x + line1.b;
    Some((x, y))
}

fn in_the_past(p: &(f64, f64), hail: &Hail) -> bool {
    let vx = p.0 - hail.px;
    let vy = p.1 - hail.py;
    let m1 = vx / hail.vx;
    let m2 = vy / hail.vy;
    m1 < 0f64 && m2 < 0f64
}

pub fn exec_day24_part1(input: &str) -> String {
    let hails = input.lines().map(|line| {
        let values = line.split(" @ ").map(|v| v.split(", ").map(|val| val.trim().parse::<i64>().unwrap() as f64).collect_vec()).collect_vec();
        Hail {
            px: values[0][0],
            py: values[0][1],
            pz: values[0][2],
            vx: values[1][0],
            vy: values[1][1],
            vz: values[1][2],
        }
    }).collect_vec();
    let lines = hails.iter().map(hail_to_line).collect_vec();

    let limit = 200000000000000f64..=400000000000000f64;
    let mut result = 0;
    for (i, l1) in lines.iter().enumerate() {
        for j in i..lines.len() {
            if let Some(a) = find_intersection(l1, &lines[j]) {
                if limit.contains(&a.0) && limit.contains(&a.1) && !in_the_past(&a, &hails[i]) && !in_the_past(&a, &hails[j]) {
                    result += 1;
                }
            }
        }
    }

    result.to_string()
}

pub fn exec_day24_part2(input: &str) -> String {
    2.to_string()
}
