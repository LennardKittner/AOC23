use std::collections::HashSet;
use itertools::Itertools;

pub fn exec_day14_part1(input: &str) -> String {
    let mut grid = input.lines().map(|c| c.as_bytes().to_vec()).collect_vec();
    let mut result = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == b'O' {
                if y == 0 {
                    result += grid.len();
                }
                for i in (0..y).rev() {
                    if grid[i][x] != b'.' {
                        grid[y][x] = b'.';
                        grid[i + 1][x] = b'O';
                        result += grid.len() - (i + 1);
                        break;
                    } else if i == 0 {
                        grid[y][x] = b'.';
                        grid[0][x] = b'O';
                        result += grid.len();
                        break;
                    }
                }
            }
        }
    }
    result.to_string()
}

pub fn exec_day14_part2(input: &str) -> String {
    let mut grid = input.lines().map(|c| c.as_bytes().to_vec()).collect_vec();
    let mut cache = HashSet::new();
    let mut loop_len = 0;
    let mut offset = 0;
    let mut loop_grid = None;
    for i in 0..1000000000 {
        cycle(&mut grid);
        if cache.contains(&grid) {
            if loop_grid.is_none() {
                loop_grid = Some(grid.clone());
                offset = i;
            } else if let Some(ref g) = loop_grid {
                if *g == grid {
                    loop_len = i - offset;
                    offset = i+1;
                    break;
                }
            }
        }
        cache.insert(grid.clone());
    }
    for _ in offset..(offset+((1000000000-offset)%loop_len)) {
        cycle(&mut grid);
    }
    grid.iter().rev().enumerate().fold(0, |acc, (i, l)| acc + l.iter().filter(|&c| c == &b'O').count() * (i + 1)).to_string()
}

fn cycle(grid: &mut [Vec<u8>]) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == b'O' {
                north(grid, y, x);
            }
        }
    }
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == b'O' {
                west(grid, y, x);
            }
        }
    }
    for y in (0..grid.len()).rev() {
        for x in 0..grid[0].len() {
            if grid[y][x] == b'O' {
                south(grid, y, x);
            }
        }
    }
    for y in 0..grid.len() {
        for x in (0..grid[0].len()).rev() {
            if grid[y][x] == b'O' {
                east(grid, y, x);
            }
        }
    }
}

fn north(grid: &mut [Vec<u8>], y: usize, x: usize) {
    for i in (0..y).rev() {
        if grid[i][x] != b'.' {
            grid[y][x] = b'.';
            grid[i + 1][x] = b'O';
            break;
        } else if i == 0 {
            grid[y][x] = b'.';
            grid[0][x] = b'O';
            break;
        }
    }
}

fn west(grid: &mut [Vec<u8>], y: usize, x: usize) {
    for i in (0..x).rev() {
        if grid[y][i] != b'.' {
            grid[y][x] = b'.';
            grid[y][i + 1] = b'O';
            break;
        } else if i == 0 {
            grid[y][x] = b'.';
            grid[y][0] = b'O';
            break;
        }
    }
}

fn south(grid: &mut [Vec<u8>], y: usize, x: usize) {
    let len = grid.len();
    for i in (y+1)..len {
        if grid[i][x] != b'.' {
            grid[y][x] = b'.';
            grid[i-1][x] = b'O';
            break;
        } else if i == len-1 {
            grid[y][x] = b'.';
            grid[len-1][x] = b'O';
            break;
        }
    }
}

fn east(grid: &mut [Vec<u8>], y: usize, x: usize) {
    let len = grid[0].len();
    for i in (x+1)..len {
        if grid[y][i] != b'.' {
            grid[y][x] = b'.';
            grid[y][i-1] = b'O';
            break;
        } else if i == len-1 {
            grid[y][x] = b'.';
            grid[y][len-1] = b'O';
            break;
        }
    }
}