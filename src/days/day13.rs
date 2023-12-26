use std::usize;

pub fn exec_day13_part1(input: &str) -> String {
    let grids = parse(input);
    let mut result = 0;
    for grid in &grids {
        let len = grid[0].len();
        let mut valid = true;
        for axis in 1..len {
            valid = true;
            for line in grid {
                let dist = usize::min(axis, len - axis);
                let mut second_half = line[(axis)..(axis + dist)].to_vec();
                second_half.reverse();
                if line[(axis - dist)..axis] != second_half {
                    valid = false;
                    break;
                }
            }
            if valid {
                result += axis;
                break;
            }
        }
        if valid {
            continue;
        }
        for axis in 1..grid.len() {
            valid = true;
            for i in 0..usize::min(axis, grid.len() - axis) {
                if grid[axis -i - 1] != grid[axis + i] {
                    valid = false;
                    break;
                }
            }
            if valid {
                result += axis * 100;
                break;
            }
        }
    }
    result.to_string()
}


fn compare(a1: &[u8], a2: &[u8]) -> i32 {
    if a1.len() != a2.len() {
        return 400;
    }
    let mut diff = 0;
    for (i, a) in a1.iter().enumerate() {
        if *a != a2[i] {
            diff += 1;
        }
        if diff > 2 {
            break;
        }
    }
   diff
}

pub fn exec_day13_part2(input: &str) -> String {
    let grids = parse(input);
    let mut result = 0;
    for grid in &grids {
        let len = grid[0].len();
        let mut valid = true;
        for axis in 1..len {
            valid = false;
            let mut diff = 0;
            for line in grid {
                let dist = usize::min(axis, len - axis);
                let mut second_half = line[(axis)..(axis + dist)].to_vec();
                second_half.reverse();
                diff += compare(&line[(axis - dist)..axis], &second_half);
            }
            if diff == 1 {
                valid = true;
                result += axis;
                break;
            }
        }
        if valid {
            continue;
        }
        for axis in 1..grid.len() {
            let mut diff = 0;
            for i in 0..usize::min(axis, grid.len() - axis) {
                diff += compare(grid[axis -i - 1], grid[axis + i]);
            }
            if diff == 1 {
                result += axis * 100;
                break;
            }
        }
    }
    result.to_string()
}

fn parse(input: &str) -> Vec<Vec<&[u8]>> {
    let mut grids = Vec::new();
    grids.push(Vec::new());
    for line in input.lines() {
        if !line.contains('#') {
            grids.push(Vec::new());
            continue;
        }
        grids.last_mut().unwrap().push(line.as_bytes());
    }
    grids
}
