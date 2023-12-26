use std::collections::HashSet;
use itertools::Itertools;

fn bfs(x: usize, y: usize, grid: &[&[u8]], steps: i32) -> u64 {
    let mut queue = HashSet::new();
    let mut next_queue = HashSet::new();
    next_queue.insert((x, y));
    for _ in 0..steps {
        queue.extend(next_queue.drain());
        for curr in queue.drain() {
            if let Some(l) = grid.get(curr.1 - 1) {
                if let Some(e) = l.get(curr.0) {
                    if *e != b'#' {
                        next_queue.insert((curr.0, curr.1 - 1));
                    }
                }
            }
            if let Some(l) = grid.get(curr.1 + 1) {
                if let Some(e) = l.get(curr.0) {
                    if *e != b'#' {
                        next_queue.insert((curr.0, curr.1 + 1));
                    }
                }
            }
            if let Some(l) = grid.get(curr.1) {
                if let Some(e) = l.get(curr.0 - 1) {
                    if *e != b'#' {
                        next_queue.insert((curr.0 - 1, curr.1));
                    }
                }
            }
            if let Some(l) = grid.get(curr.1) {
                if let Some(e) = l.get(curr.0 + 1) {
                    if *e != b'#' {
                        next_queue.insert((curr.0 + 1, curr.1));
                    }
                }
            }
        }
        //println!("{}", next_queue.len());
        //println!("{next_queue:?}");
        // let mut grid2 = grid.to_vec().iter().map(|a| a.to_vec()).collect_vec();
        // for coordinate in &next_queue {
        //     grid2[coordinate.1][coordinate.0] = b'O';
        // }
        //println!("{}", grid2.iter().map(|line| line.iter().map(|c| *c as char).join("")).join("\n"));
    }
    next_queue.len() as u64
}


pub fn exec_day21_part1(input: &str) -> String {
    let grid = input.lines().map(|line| line.as_bytes()).collect_vec();
    let mut start = (0, 0);
    for (y, line) in grid.iter().enumerate() {
        for (x , c) in line.iter().enumerate() {
            if *c == b'S' {
                start = (x, y);
                break;
            }
        }
    }
    bfs(start.0, start.1, &grid, 64).to_string()
}

pub fn exec_day21_part2(input: &str) -> String {
    todo!("{}", input.len())
}
