use std::cmp::Ordering;
use std::collections::HashMap;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use crate::days::day17::Direction::{Down, Left, Right, Up};

pub fn exec_day17_part1(input: &str) -> String {
    let grid = input.lines().map(|l| l.as_bytes().iter().map(|b| b - b'0').collect_vec()).collect_vec();
    dijkstra(&grid, check_point).to_string()
}

pub fn exec_day17_part2(input: &str) -> String {
    let grid = input.lines().map(|l| l.as_bytes().iter().map(|b| b - b'0').collect_vec()).collect_vec();
    dijkstra(&grid, ultra_check_point).to_string()
}

type State = (usize, usize, i32, Direction);
type CheckPointFn = fn(grid: &[Vec<u8>], distances: &mut HashMap<State, (u64, State)>, queue: &mut PriorityQueue<State, Distance>, curr_distance: &Distance, curr: &State, direction: Direction) -> ();

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Distance {
    distance: u64,
}

impl Eq for Distance {}

impl PartialEq<Self> for Distance {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl PartialOrd<Self> for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

impl From<u64> for Distance {
    fn from(value: u64) -> Self {
        Distance {distance: value}
    }
}

fn dijkstra(grid: &[Vec<u8>], check_point: CheckPointFn) -> u64 {
    let mut distances = HashMap::new();
    let mut queue = PriorityQueue::new();
    queue.push((0, 0, 0, Up), Distance {distance: 0});
    distances.insert((0, 0, 0, Up), (0u64, (0, 0, 0, Down)));
    distances.insert((0, 0, 0, Down), (0u64, (0, 0, 0, Down)));
    distances.insert((0, 0, 0, Left), (0u64, (0, 0, 0, Down)));
    distances.insert((0, 0, 0, Right), (0u64, (0, 0, 0, Down)));
    while let Some((curr, distance)) = queue.pop() {
        check_point(grid, &mut distances, &mut queue, &distance, &curr, Up);
        check_point(grid, &mut distances, &mut queue, &distance, &curr, Down);
        check_point(grid, &mut distances, &mut queue, &distance, &curr, Left);
        check_point(grid, &mut distances, &mut queue, &distance, &curr, Right);
    }
    let keys = distances.keys().filter(|k| k.1 ==  grid.len()-1 && k.0 == grid[0].len()-1).collect_vec();
    let keys = keys.iter().map(|k| distances[k].clone()).collect_vec();
    let result = keys.iter().min_by_key(|e| e.0).unwrap();
    // visualisation of the path
    // let mut grid2 = vec![vec![b'#'; grid[0].len()]; grid.len()];
    // let mut curr = result.1.clone();
    // grid2[0][0] = b'O';
    // grid2[grid.len()-1][grid[0].len()-1] = b'O';
    // while curr.0 != 0 || curr.1 != 0 {
    //     grid2[curr.1][curr.0] = b'O';
    //     curr = distances[&curr].clone().1;
    // }
    //println!("{}", grid2.iter().map(|line| line.iter().map(|c| *c as char).join("")).join("\n"));
    result.0
}

fn check_point(grid: &[Vec<u8>], distances: &mut HashMap<State, (u64, State)>, queue: &mut PriorityQueue<State, Distance>, curr_distance: &Distance, curr: &State, direction: Direction) {
    match (&direction, &curr.3) {
        (Left, Right) => return,
        (Right, Left) => return,
        (Up, Down) => return,
        (Down, Up) => return,
        _ => ()
    }
    let mut point = (curr.0, curr.1, if direction == curr.3 { curr.2 + 1 } else { 1 }, direction.clone());
    match &direction {
        Left => point.0 = point.0.checked_sub(1).unwrap_or(usize::MAX),
        Right => point.0 += 1,
        Up => point.1 = point.1.checked_sub(1).unwrap_or(usize::MAX),
        Down => point.1 += 1,
    }
    if point.2 > 3 || !(0..grid.len()).contains(&point.1) || !(0..grid[0].len()).contains(&point.0) {
        return;
    }
    let new_distance = curr_distance.distance + grid[point.1][point.0] as u64;
    if let Some((mind_dis, _)) = distances.get(&point) {
        if new_distance < *mind_dis {
            queue.push_decrease(point.clone(), new_distance.into());
            distances.insert(point, (new_distance, curr.clone()));
        }
    } else {
        queue.push_decrease(point.clone(), new_distance.into());
        distances.insert(point, (new_distance, curr.clone()));
    }
}

fn ultra_check_point(grid: &[Vec<u8>], distances: &mut HashMap<State, (u64, State)>, queue: &mut PriorityQueue<State, Distance>, curr_distance: &Distance, curr: &State, direction: Direction) {
    match (&direction, &curr.3) {
        (Left, Right) => return,
        (Right, Left) => return,
        (Up, Down) => return,
        (Down, Up) => return,
        _ => ()
    }
    let mut point = (curr.0, curr.1, 1, direction.clone());
    if curr.3 == direction {
        if curr.2 >= 10 {
            return;
        }
        match &direction {
            Left => point.0 = point.0.checked_sub(1).unwrap_or(usize::MAX),
            Right => point.0 += 1,
            Up => point.1 = point.1.checked_sub(1).unwrap_or(usize::MAX),
            Down => point.1 += 1,
        }
        point.2 = curr.2 + 1;
    } else {
        match &direction {
            Left => point.0 = point.0.checked_sub(4).unwrap_or(usize::MAX),
            Right => point.0 += 4,
            Up => point.1 = point.1.checked_sub(4).unwrap_or(usize::MAX),
            Down => point.1 += 4,
        }
        point.2 = 4;
    }
    if !(0..grid.len()).contains(&point.1) || !(0..grid[0].len()).contains(&point.0) {
        return;
    }
    let new_distance = if curr.3 == direction {
        curr_distance.distance + grid[point.1][point.0] as u64
    } else {
        curr_distance.distance + match &direction {
            Left => (grid[curr.1][curr.0-1] + grid[curr.1][curr.0-2] + grid[curr.1][curr.0-3] + grid[curr.1][curr.0-4]) as u64,
            Right => (grid[curr.1][curr.0+1] + grid[curr.1][curr.0+2] + grid[curr.1][curr.0+3] + grid[curr.1][curr.0+4]) as u64,
            Up => (grid[curr.1-1][curr.0] + grid[curr.1-2][curr.0] + grid[curr.1-3][curr.0] + grid[curr.1-4][curr.0]) as u64,
            Down => (grid[curr.1+1][curr.0] + grid[curr.1+2][curr.0] + grid[curr.1+3][curr.0] + grid[curr.1+4][curr.0]) as u64,
        }
    };
    if let Some((mind_dis, _)) = distances.get(&point) {
        if new_distance < *mind_dis {
            queue.push_decrease(point.clone(), new_distance.into());
            distances.insert(point, (new_distance, curr.clone()));
        }
    } else {
        queue.push_decrease(point.clone(), new_distance.into());
        distances.insert(point, (new_distance, curr.clone()));
    }
}
