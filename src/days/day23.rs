use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn exec_day23_part1(input: &str) -> String {
    let (grid, start, end) = parse(input);
    let mut cache: HashMap<(usize, (usize, usize)), u64> = HashMap::new();
    let mut max = 0;
    longest_path(&mut max, true, &grid, &HashSet::new(), start, end, &mut cache).unwrap().to_string()
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

fn longest_path(max: &mut u64, part1: bool, grid: &[&[u8]], visited: &HashSet<(usize, usize)>, start: (usize, usize), end: (usize, usize), cache: &mut HashMap<(usize, (usize, usize)), u64>) -> Option<u64> {
    if start == end {
        // println!("{}", visited.len());
        // let mut grid = grid.to_vec().iter().map(|line| line.to_vec()).collect_vec();
        // for (x, y) in visited {
        //     grid[*y][*x] = b'O';
        // }
        // println!("{}", grid.iter().map(|line| line.iter().map(|c| *c as char).join("")).join("\n"));
        if visited.len() as u64 > *max {
            *max = visited.len() as u64;
        }
        return Some(visited.len() as u64)
    }
    let mut paths = Vec::new();
    paths.push(check_point(max, part1, grid, visited, end, (start.0 as i64 -1, start.1 as i64 ), if part1 { b'>' } else { b'#' }, cache).unwrap_or(0));
    paths.push(check_point(max, part1, grid, visited, end, (start.0 as i64 +1, start.1 as i64 ), if part1 { b'<' } else { b'#' }, cache).unwrap_or(0));
    paths.push(check_point(max, part1, grid, visited, end, (start.0 as i64 , start.1 as i64 -1), if part1 { b'v' } else { b'#' }, cache).unwrap_or(0));
    paths.push(check_point(max, part1, grid, visited, end, (start.0 as i64 , start.1 as i64 +1), b'#', cache).unwrap_or(0));
    if let Some(max) = paths.iter().max() {
        cache.insert((visited.len(), start), *max);
        Some(*max)
    } else {
        None
    }
}

fn check_point(max: &mut u64, part1: bool, grid: &[&[u8]], visited: &HashSet<(usize, usize)>, end: (usize, usize), point: (i64, i64), forbidden: u8, cache: &mut HashMap<(usize, (usize, usize)), u64>) -> Option<u64> {
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
                return longest_path(max, part1, grid, &visited, point, end, cache);
            }
        }
    }
    None
}

pub fn exec_day23_part2(input: &str) -> String {
    let (grid, start, end) = parse(input);
    let graph = shrink_graph(&grid, &start, &end);
    let mut g = graph.iter().collect_vec();
    g.sort();
    let a = longest_path2(&graph, &start, &end, 0);
    a.0.to_string()
}

fn longest_path2(graph: &HashMap<(usize, usize), Vec<((usize, usize), u64)>>, start: &(usize, usize), end: &(usize, usize), curr_len: u64) -> (u64, Vec<(usize, usize)>) {
    if start == end {
        return (curr_len, Vec::new());
    }
    if !graph.contains_key(start) {
        return (0, Vec::new());
    }
    let mut graph = graph.clone();
    let children = graph.remove(start).unwrap();
    let mut paths = Vec::new();
    for child in children {
        paths.push(longest_path2(&graph, &child.0, end, curr_len + child.1));
    }
    let result = paths.iter().max_by_key(|e| e.0).unwrap();
    let mut tmp = result.1.clone();
    tmp.insert(0, *start);
    (result.0, tmp)
}

fn shrink_graph(grid: &[&[u8]], start: &(usize, usize), end: &(usize, usize)) -> HashMap<(usize, usize), Vec<((usize, usize), u64)>> {
    let mut shrink_graph = HashMap::new();
    shrink_graph.insert(*start, Vec::new());
    shrink_graph.insert(*end, Vec::new());
    let mut visited = HashSet::new();
    let mut check_next = Vec::new();
    visited.insert(*start);
    check_next.push(*start);
    while let Some(node) = check_next.pop() {
        update(grid, end, &mut shrink_graph, &mut visited, &mut check_next, &node,  &(node.0.checked_sub(1).unwrap_or(usize::MAX), node.1));
        update(grid, end, &mut shrink_graph, &mut visited, &mut check_next, &node,  &(node.0+1, node.1));
        update(grid, end, &mut shrink_graph, &mut visited, &mut check_next, &node,  &(node.0, node.1.checked_sub(1).unwrap_or(usize::MAX)));
        update(grid, end, &mut shrink_graph, &mut visited, &mut check_next, &node,  &(node.0, node.1+1));
    }
    shrink_graph
}

fn update(grid: &[&[u8]], end: &(usize, usize), shrink_graph: &mut HashMap<(usize, usize), Vec<((usize, usize), u64)>>, mut visited: &mut HashSet<(usize, usize)>, check_next: &mut Vec<(usize, usize)>, node: &(usize, usize), point: &(usize, usize)) {
    if let Some(next) = explore(grid, &mut visited, point, end, node) {
        if shrink_graph.contains_key(&next.0) {
            shrink_graph.get_mut(&next.0).unwrap().push((*node, next.1));
        } else {
            check_next.push(next.0);
            shrink_graph.insert(next.0, vec![(*node, next.1)]);
        }
        shrink_graph.get_mut(&node).unwrap().push(next);
    }
}

fn explore(grid: &[&[u8]], visited: &mut HashSet<(usize, usize)>, start: &(usize, usize), end: &(usize, usize), crossing: &(usize, usize)) -> Option<((usize, usize), u64)> {
    if !(0..grid.len()).contains(&start.1) || !(0..grid[0].len()).contains(&start.0) {
        return None;
    }
    if grid[start.1][start.0] == b'#' {
        return None;
    }
    let mut curr = *start;
    visited.insert(curr);
    let mut distance = 1;
    while !is_crossing(grid, &curr, end) {
        if let Some(next) = get_next(grid, visited, &curr, end, crossing) {
            curr = next;
            distance += 1;
            visited.insert(next);
        } else {
            return None;
        }
    }
    Some((curr, distance))
}

fn is_crossing(grid: &[&[u8]], point: &(usize, usize), end: &(usize, usize)) -> bool {
    if point == end {
        return true;
    }
    let mut neighbors = Vec::new();
    neighbors.push(get(grid, &(point.0.checked_sub(1).unwrap_or(usize::MAX), point.1)));
    neighbors.push(get(grid, &(point.0+1, point.1)));
    neighbors.push(get(grid, &(point.0, point.1.checked_sub(1).unwrap_or(usize::MAX))));
    neighbors.push(get(grid, &(point.0, point.1+1)));
    neighbors.iter().filter(|c| **c == b'#').count() < 2
}

fn get(grid: &[&[u8]], point: &(usize, usize)) -> u8 {
    if let Some(tmp) = grid.get(point.1) {
        if let Some(c) = tmp.get(point.0) {
            return *c;
        }
    }
    b'#'
}

fn get_next(grid: &[&[u8]], visited: &HashSet<(usize, usize)>, point: &(usize, usize), end: &(usize, usize), crossing: &(usize, usize)) -> Option<(usize, usize)> {
    let p1 = &(point.0, point.1.checked_sub(1).unwrap_or(usize::MAX));
    let p2 = &(point.0, point.1+1);
    let p3 = &(point.0.checked_sub(1).unwrap_or(usize::MAX), point.1);
    let p4 = &(point.0+1, point.1);

    if crossing != p1 && get(grid, p1) != b'#' && (is_crossing(grid, p1, end) || !visited.contains(p1)) {
        Some(*p1)
    } else if crossing != p2 && get(grid, p2) != b'#' && (is_crossing(grid, p2, end) || !visited.contains(p2)) {
        Some(*p2)
    } else if crossing != p3 && get(grid, p3) != b'#' && (is_crossing(grid, p3, end) || !visited.contains(p3)) {
        Some(*p3)
    } else if crossing != p4 && get(grid, p4) != b'#' && (is_crossing(grid, p4, end) || !visited.contains(p4)) {
        Some(*p4)
    } else {
        None
    }
}