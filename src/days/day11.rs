use std::collections::HashSet;
use itertools::Itertools;

#[derive(Clone)]
struct Entry {
    galaxy: bool,
    marked: bool,
    distance: u64,
}

pub fn exec_day11_part1(input: &str) -> String {
    let lines = input.lines().map(|l| l.as_bytes().iter().dedup_with_count().collect_vec()).collect_vec();
    let mut grid = input.lines().map(|s| s.as_bytes().iter().map(|c| Entry {
        galaxy: *c == b'#',
        marked: false,
        distance: 0,
    } ).collect_vec()).collect_vec();
    for i in (0..lines.len()).rev() {
        if lines[i].len() == 1 {
            grid.insert(i+1, vec![Entry {
                galaxy: false,
                marked: false,
                distance: 0,
            }; grid[0].len()])
        }
    }
    for x in (0..grid[0].len()).rev() {
        let mut all_empty = true;
        for y in 0..grid.len() {
            if grid[y][x].galaxy {
                all_empty = false;
                break;
            }
        }
        if all_empty {
            for y in 0..grid.len() {
                grid[y].insert(x+1, Entry {
                    galaxy: false,
                    marked: false,
                    distance: 0,
                })
            }
        }
    }
    // println!("{}", grid.iter().map(|l| l.iter().map(|e| if e.galaxy {'#'} else {'.'}).join("")).join("\n"));
    let mut result = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if !grid[y][x].galaxy {
                continue;
            }
            result += bfs(x, y, &grid);
            grid[y][x].galaxy = false;
        }
    }
    // println!("{}", grid.iter().map(|l| l.iter().map(|e| if e.galaxy {'#'} else {'.'}).join("")).join("\n"));
    result.to_string()
}

fn bfs(x: usize, y: usize, grid: &Vec<Vec<Entry>>) -> u64 {
    let mut result = 0;
    let mut working_grid = grid.clone();
    let mut queue = Vec::new();
    queue.push((x, y));
    working_grid[y][x].marked = true;
    while !queue.is_empty() {
        let curr = *queue.first().unwrap();
        queue.remove(0);
        let depth = working_grid[curr.1][curr.0].distance;
        if working_grid[curr.1][curr.0].galaxy {
            result += depth;
        }
        if let Some(l) = working_grid.get_mut(curr.1-1) {
            if let Some(e) = l.get_mut(curr.0) {
                if !e.marked {
                    queue.push((curr.0, curr.1-1));
                    e.distance = depth + 1;
                    e.marked = true;
                }
            }
        }
        if let Some(l) = working_grid.get_mut(curr.1+1) {
            if let Some(e) = l.get_mut(curr.0) {
                if !e.marked {
                    queue.push((curr.0, curr.1+1));
                    e.distance = depth + 1;
                    e.marked = true;
                }
            }
        }
        if let Some(l) = working_grid.get_mut(curr.1) {
            if let Some(e) = l.get_mut(curr.0-1) {
                if !e.marked {
                    queue.push((curr.0-1, curr.1));
                    e.distance = depth + 1;
                    e.marked = true;
                }
            }
        }
        if let Some(l) = working_grid.get_mut(curr.1) {
            if let Some(e) = l.get_mut(curr.0+1) {
                if !e.marked {
                    queue.push((curr.0+1, curr.1));
                    e.distance = depth + 1;
                    e.marked = true;
                }
            }
        }
    }
    result
}

pub fn exec_day11_part2(input: &str) -> String {
    let lines = input.lines().map(|l| l.as_bytes().iter().dedup_with_count().collect_vec()).collect_vec();
    let grid = input.lines().map(|s| s.as_bytes().iter().map(|c| Entry {
        galaxy: *c == b'#',
        marked: false,
        distance: 0,
    } ).collect_vec()).collect_vec();
    let mut rows = HashSet::new();
    let mut columns = HashSet::new();
    let mut galaxies = Vec::new();
    for i in (0..lines.len()).rev() {
        if lines[i].len() == 1 {
            rows.insert(i);
        }
    }
    for x in (0..grid[0].len()).rev() {
        let mut all_empty = true;
        for y in 0..grid.len() {
            if grid[y][x].galaxy {
                galaxies.push((x, y));
                all_empty = false;
            }
        }
        if all_empty {
            columns.insert(x);
        }
    }
    let mut result: u64 = 0;
    while let Some(g1) = galaxies.pop() {
        for g2 in &galaxies {
            let mut dst_y = 0;
            let mut dst_x = 0;
            for x in g1.0.min(g2.0)..g1.0.max(g2.0) {
                dst_x += if columns.contains(&x) { 1000000 } else { 1 }
            }
            for y in g1.1.min(g2.1)..g1.1.max(g2.1) {
                dst_y += if rows.contains(&y) { 1000000 } else { 1 }
            }
            result += dst_x + dst_y;
        }
    }
    result.to_string()
}
