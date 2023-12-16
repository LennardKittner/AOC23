use itertools::Itertools;

#[derive(Debug, Clone)]
struct Beam {
    x: i32,
    y: i32,
    dir_x: i32,
    dir_y: i32,
}

// This code assumes that 800 iterations are sufficient to calculate the energized fields. It also does not check the corners.

pub fn exec_day16_part1(input: &str) -> String {
    let mut grid = input.lines().map(|s| s.as_bytes().iter().map(|b| (b, false)).collect_vec()).collect_vec();
    simulate_beam(&mut grid, &Beam {
        x: 0,
        y: 0,
        dir_x: 1,
        dir_y: 0,
    }).to_string()
}

pub fn exec_day16_part2(input: &str) -> String {
    let mut grid = input.lines().map(|s| s.as_bytes().iter().map(|b| (b, false)).collect_vec()).collect_vec();
    let default = grid.clone();
    let mut result = 0;
    for i in 0..grid.len() {
        grid = default.clone();
        result = result.max(simulate_beam(&mut grid, &Beam {
            x: 0,
            y: i as i32,
            dir_x: 1,
            dir_y: 0,
        }));
    }
    for i in 0..grid.len() {
        grid = default.clone();
        result = result.max(simulate_beam(&mut grid, &Beam {
            x: (default[0].len() - 1) as i32,
            y: i as i32,
            dir_x: -1,
            dir_y: 0,
        }));
    }
    for i in 0..grid[0].len() {
        grid = default.clone();
        result = result.max(simulate_beam(&mut grid, &Beam {
            x: i as i32,
            y: 0,
            dir_x: 0,
            dir_y: 1,
        }));
    }
    for i in 0..grid[0].len() {
        grid = default.clone();
        result = result.max(simulate_beam(&mut grid, &Beam {
            x: i as i32,
            y: (default.len() - 1) as i32,
            dir_x: 0,
            dir_y: -1,
        }));
    }
    result.to_string()
}

fn simulate_beam(grid: &mut [Vec<(&u8, bool)>], start: &Beam) -> usize {
    let mut beams = Vec::new();
    beams.push(start.clone());
    for _ in 0..800 {
        let mut new_beams = Vec::new();
        for beam in beams.iter_mut() {
            grid[beam.y as usize][beam.x as usize].1 = true;
            let new_direction1: (i32, i32) = match (grid[beam.y as usize][beam.x as usize].0, (beam.dir_x, beam.dir_y)) {
                (b'/', (1, 0)) => (0, -1),
                (b'/', (-1, 0)) => (0, 1),
                (b'/', (0, 1)) => (-1, 0),
                (b'/', (0, -1)) => (1, 0),
                (b'\\', (1, 0)) => (0, 1),
                (b'\\', (-1, 0)) => (0, -1),
                (b'\\', (0, 1)) => (1, 0),
                (b'\\', (0, -1)) => (-1, 0),
                (b'-', (1, 0)) => (1, 0),
                (b'-', (-1, 0)) => (-1, 0),
                (b'-', (0, 1)) => (-1, 0),
                (b'-', (0, -1)) => (-1, 0),
                (b'|', (1, 0)) => (0, -1),
                (b'|', (-1, 0)) => (0, -1),
                (b'|', (0, 1)) => (0, 1),
                (b'|', (0, -1)) => (0, -1),
                (b'.', d) => d,
                e => panic!("impossible {} {} {}", *e.0 as char, e.1.0, e.1.0),
            };
            let new_direction2: (i32, i32) = match (grid[beam.y as usize][beam.x as usize].0, (beam.dir_x, beam.dir_y)) {
                (b'-', (0, 1)) => (1, 0),
                (b'-', (0, -1)) => (1, 0),
                (b'|', (1, 0)) => (0, 1),
                (b'|', (-1, 0)) => (0, 1),
                _ => (-1, -1),
            };
            beam.x += new_direction1.0;
            beam.y += new_direction1.1;
            beam.dir_x = new_direction1.0;
            beam.dir_y = new_direction1.1;
            if new_direction2 != (-1, -1) {
                new_beams.push(Beam {
                    x: beam.x + new_direction2.0,
                    y: beam.y + new_direction2.1,
                    dir_x: new_direction2.0,
                    dir_y: new_direction2.1,
                })
            }
        }
        beams.append(&mut new_beams);
        beams.retain(|beam| (0..(grid.len() as i32)).contains(&beam.y) && (0..(grid[0].len() as i32)).contains(&beam.x))
    }
    grid.iter().map(|l| l.iter().filter(|e| e.1).count()).sum::<usize>()
}
