use std::cmp::Ordering;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use log::log;

pub fn exec_day17_part1(input: &str) -> String {
    let grid = input.lines().map(|l| l.as_bytes().iter().map(|b| b - b'0').collect_vec()).collect_vec();
    dijkstra(&grid).to_string()
}

pub fn exec_day17_part2(input: &str) -> String {
    0.to_string()
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

fn dijkstra(grid: &[Vec<u8>]) -> u64 {
    let mut marks_and_distance = vec![vec![(u64::MAX, false, (0, 0)); grid[0].len()]; grid.len()];
    let mut queue = PriorityQueue::new();
    queue.push((0, 0), Distance {distance: 0});
    marks_and_distance[0][0] = (0, true, (0, 0));
    while !queue.is_empty() {
        let curr = queue.pop().unwrap().0;
        let distance = marks_and_distance[curr.1][curr.0].0;
        let aa = marks_and_distance[curr.1][curr.0].2;
        let aaa = marks_and_distance[aa.1][aa.0].2;
        println!("{:?}, {:?}, {:?}", curr, aa, aaa);
        if let Some(l) = marks_and_distance.get_mut(curr.1-1) {
            if let Some(e) = l.get_mut(curr.0) {
                let new_distance = distance + grid[curr.1-1][curr.0] as u64;
                let next = (curr.0, curr.1-1);
                if aa == (0, 0) || !((next.0 == aa.0 && aa.0 == aaa.0 && aaa.0 == curr.0) || (next.1 == aa.1 && aa.1 == aaa.1 && aaa.1 == curr.1)) {
                    if !e.1 || new_distance < e.0 {
                        println!("{:?} {:?}, {:?}, {:?}", e.2, curr, aa, aaa);
                        queue.push_decrease((curr.0, curr.1 - 1), new_distance.into());
                        e.0 = new_distance;
                        e.1 = true;
                        e.2 = (curr.0, curr.1);
                    }
                }
            }
        }
        if let Some(l) = marks_and_distance.get_mut(curr.1+1) {
            if let Some(e) = l.get_mut(curr.0) {
                let new_distance = distance + grid[curr.1+1][curr.0] as u64;
                let next = (curr.0, curr.1+1);
                if aa == (0, 0) || !((next.0 == aa.0 && aa.0 == aaa.0 && aaa.0 == curr.0) || (next.1 == aa.1 && aa.1 == aaa.1 && aaa.1 == curr.1)) {
                    if !e.1 || new_distance < e.0 {
                        println!("{:?} {:?}, {:?}, {:?}", e.2, curr, aa, aaa);
                        queue.push_decrease((curr.0, curr.1 + 1), new_distance.into());
                        e.0 = new_distance;
                        e.1 = true;
                        e.2 = (curr.0, curr.1);
                    }
                }
            }
        }
        if let Some(l) = marks_and_distance.get_mut(curr.1) {
            if let Some(e) = l.get_mut(curr.0-1) {
                let new_distance = distance + grid[curr.1][curr.0-1] as u64;
                let next = (curr.0-1, curr.1);
                if aa == (0, 0) || !((next.0 == aa.0 && aa.0 == aaa.0 && aaa.0 == curr.0) || (next.1 == aa.1 && aa.1 == aaa.1 && aaa.1 == curr.1)) {
                    if !e.1 || new_distance < e.0 {
                        println!("{:?} {:?}, {:?}, {:?}", e.2, curr, aa, aaa);
                        queue.push_decrease((curr.0 - 1, curr.1), new_distance.into());
                        e.0 = new_distance;
                        e.1 = true;
                        e.2 = (curr.0, curr.1);
                    }
                }
            }
        }
        if let Some(l) = marks_and_distance.get_mut(curr.1) {
            if let Some(e) = l.get_mut(curr.0+1) {
                let new_distance = distance + grid[curr.1][curr.0+1] as u64;
                let next = (curr.0+1, curr.1);
                if aa == (0, 0) || !((next.0 == aa.0 && aa.0 == aaa.0 && aaa.0 == curr.0) || (next.1 == aa.1 && aa.1 == aaa.1 && aaa.1 == curr.1)) {
                    if !e.1 || new_distance < e.0 {
                        println!("{:?} {:?}, {:?}, {:?}", e.2, curr, aa, aaa);
                        queue.push_decrease((curr.0 + 1, curr.1), new_distance.into());
                        e.0 = new_distance;
                        e.1 = true;
                        e.2 = (curr.0, curr.1);
                    }
                }
            }
        }
        //println!("{}\n--------------", marks_and_distance.iter().map(|l| format!("{:?}", l)).join("\n"));
    }
    let mut g2 = grid.clone().to_vec();
    let mut curr = (g2[0].len()-1, g2.len()-1);
    loop {
        g2[curr.1][curr.0] = 0;
        if curr == (0, 0) {
            break;
        }
        curr = marks_and_distance[curr.1][curr.0].2;
    }
    println!("{}", g2.iter().map(|l| format!("{:?}", l)).join("\n"));

    marks_and_distance.last().unwrap().last().unwrap().0
}
