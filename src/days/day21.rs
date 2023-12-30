use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn bfs(x: usize, y: usize, grid: &[&[u8]], steps: i32) -> HashMap<(usize, usize), i32> {
    let mut queue = HashSet::new();
    let mut distances = HashMap::new();
    let mut next_queue = HashSet::new();
    distances.insert((x, y), 0);
    next_queue.insert((x, y));
    for step in 1..=steps {
        queue.extend(next_queue.drain());
        for curr in queue.drain() {
            check_point(grid, &mut distances, &mut next_queue, step, &(curr.0, curr.1-1));
            check_point(grid, &mut distances, &mut next_queue, step, &(curr.0, curr.1+1));
            check_point(grid, &mut distances, &mut next_queue, step, &(curr.0-1, curr.1));
            check_point(grid, &mut distances, &mut next_queue, step, &(curr.0+1, curr.1));
        }
    }
    distances
}

fn check_point(grid: &[&[u8]], distances: &mut HashMap<(usize, usize), i32>, next_queue: &mut HashSet<(usize, usize)>, step: i32, curr: &(usize, usize)) {
    if let Some(l) = grid.get(curr.1) {
        if let Some(e) = l.get(curr.0) {
            if *e != b'#' && !distances.contains_key(&(curr.0, curr.1)) {
                next_queue.insert((curr.0, curr.1));
                distances.insert((curr.0, curr.1), step);
            }
        }
    }
}


pub fn exec_day21_part1(input: &str) -> String {
    let (grid, start) = parse(input);
    bfs(start.0, start.1, &grid, 64).values().filter(|d| **d % 2 == 0).count().to_string()
}

fn parse(input: &str) -> (Vec<&[u8]>, (usize, usize)) {
    let grid = input.lines().map(|line| line.as_bytes()).collect_vec();
    let mut start = (0, 0);
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == b'S' {
                start = (x, y);
                break;
            }
        }
    }
    (grid, start)
}

// part 2 was heavily inspired by https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
pub fn exec_day21_part2(input: &str) -> String {
    let (grid, start) = parse(input);
    let distances = bfs(start.0, start.1, &grid, 1000);
    let even_corners = distances.values().filter(|&&v| v % 2 == 0 && v > 65).count();
    let odd_corners = distances.values().filter(|&&v| v % 2 == 1 && v > 65).count();
    let even_total = distances.values().filter(|&&v| v % 2 == 0).count();
    let odd_total = distances.values().filter(|&&v| v % 2 == 1).count();

    let n = 202300;
    (((n+1)*(n+1)) * odd_total + (n*n) * even_total - (n+1) * odd_corners + n * even_corners).to_string()
}
