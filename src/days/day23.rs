use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn exec_day23_part1(input: &str) -> String {
    let (grid, start, end) = parse(input);
    let mut cache: HashMap<(usize, (usize, usize)), u64> = HashMap::new();
    longest_path(true, &grid, &HashSet::new(), start, end, &mut cache).unwrap().to_string()
}

fn parse(input: &str) -> (Vec<&[u8]>, (usize, usize), (usize, usize)) {
    let grid = input.lines().map(|line| line.as_bytes()).collect_vec();
    let mut start = (0usize, 0usize);
    let mut end = (0usize, 0usize);
    for (i, c) in grid[0].iter().enumerate() {
        if *c == b'.' {
            start = (i, 0);
            break;
        }
    }
    for (i, c) in grid.last().unwrap().iter().enumerate() {
        if *c == b'.' {
            end = (i, grid.len() - 1);
            break;
        }
    }
    (grid, start, end)
}

fn longest_path(part1: bool, grid: &[&[u8]], visited: &HashSet<(usize, usize)>, start: (usize, usize), end: (usize, usize), cache: &mut HashMap<(usize, (usize, usize)), u64>) -> Option<u64> {
    if start == end {
        // println!("{}", visited.len());
        // let mut grid = grid.to_vec().iter().map(|line| line.to_vec()).collect_vec();
        // for (x, y) in visited {
        //     grid[*y][*x] = b'O';
        // }
        // println!("{}", grid.iter().map(|line| line.iter().map(|c| *c as char).join("")).join("\n"));
        return Some(visited.len() as u64)
    }
    let mut paths = Vec::new();
    paths.push(check_point(part1, grid, visited, end, (start.0 as i64 -1, start.1 as i64 ), if part1 { b'>' } else { b'#' }, cache).unwrap_or(0));
    paths.push(check_point(part1, grid, visited, end, (start.0 as i64 +1, start.1 as i64 ), if part1 { b'<' } else { b'#' }, cache).unwrap_or(0));
    paths.push(check_point(part1, grid, visited, end, (start.0 as i64 , start.1 as i64 -1), if part1 { b'v' } else { b'#' }, cache).unwrap_or(0));
    paths.push(check_point(part1, grid, visited, end, (start.0 as i64 , start.1 as i64 +1), b'#', cache).unwrap_or(0));
    if let Some(max) = paths.iter().max() {
        cache.insert((visited.len(), start), *max);
        Some(*max)
    } else {
        None
    }
}

fn check_point(part1: bool, grid: &[&[u8]], visited: &HashSet<(usize, usize)>, end: (usize, usize), point: (i64, i64), forbidden: u8, cache: &mut HashMap<(usize, (usize, usize)), u64>) -> Option<u64> {
    if point.0 < 0 || point.1 < 0 {
        return None;
    }
    let point = (point.0 as usize, point.1 as usize);
    if let Some(len) = cache.get(&(visited.len(), point)) {
        return Some(*len);
    }
    if let Some(tmp) = grid.get(point.1) {
        if let Some(next) = tmp.get(point.0) {
            if !visited.contains(&point) && *next != b'#' && *next != forbidden {
                let mut visited = visited.clone();
                let mut point = point;
                if part1 {
                    visited.insert(point);
                    point = match next {
                        b'v' => (point.0, point.1 + 1),
                        b'<' => (point.0 - 1, point.1),
                        b'>' => (point.0 + 1, point.1),
                        _ => point,
                    };
                }
                visited.insert(point);
                return longest_path(part1, grid, &visited, point, end, cache);
            }
        }
    }
    None
}

pub fn exec_day23_part2(input: &str) -> String {
    let mut cache: HashMap<(usize, (usize, usize)), u64> = HashMap::new();
    let (grid, start, end) = parse(input);
    longest_path(false, &grid, &HashSet::new(), start, end, &mut cache).unwrap().to_string()
}
