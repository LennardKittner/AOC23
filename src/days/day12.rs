use std::collections::HashMap;
use itertools::Itertools;

fn combinations(cache: &mut HashMap<(usize, usize), u64>, springs: &[u8], nums: &[usize]) -> u64 {
    if nums.is_empty() {
        return if springs.contains(&b'#') { 0 } else { 1 };
    } else if springs.is_empty() || nums[0] > springs.len() {
        return 0;
    }
    if let Some(res) = cache.get(&(springs.len(), nums.len())) {
        return *res;
    }
    let mut springs = springs.to_vec();
    let result = match springs[0] {
        b'.' => {
            combinations(cache, &springs[1..], nums)
        },
        b'#' => {
            if !springs[..nums[0]].contains(&b'.') && (springs.len() == nums[0] || springs[nums[0]] != b'#') {
                combinations(cache, if springs.len() == nums[0] { &[] } else { &springs[(nums[0]+1)..] }, &nums[1..])
            } else {
                0
            }
        },
        b'?' => {
            springs[0] = b'#';
            let mut result = 0;
            if !springs[..nums[0]].contains(&b'.') && (springs.len() == nums[0] || springs[nums[0]] != b'#') {
                result += combinations(cache, if springs.len() == nums[0] { &[] } else { &springs[(nums[0]+1)..] }, &nums[1..])
            }
            springs[0] = b'.';
            result += combinations(cache, if springs.len() == 1 { &[] } else { &springs[1..] }, nums);
            result
        },
        _ => panic!("unknons symbol")
    };
    cache.insert((springs.len(), nums.len()), result);
    result
}

pub fn exec_day12_part1(input: &str) -> String {
    let lines = input.lines()
    .map(|l| l.split(' ').collect_vec())
    .fold(Vec::new(), |mut a, l| {
        let springs = l[0].as_bytes().to_vec();
        let nums = l[1].split(',').map(|n| n.parse::<usize>().unwrap()).collect_vec();
        a.push((springs, nums));
        a
    });
    day12(lines).to_string()
}

fn day12(lines: Vec<(Vec<u8>, Vec<usize>)>) -> u64 {
    let mut result = 0;
    for line in lines.iter() {
        let mut cache = HashMap::new();
        result += combinations(&mut cache, &line.0, &line.1);
    }
    result
}

pub fn exec_day12_part2(input: &str) -> String {
    let lines = input.lines()
        .map(|l| l.split(' ').collect_vec())
        .fold(Vec::new(), |mut a, l| {
            let springs = format!("{0}?{0}?{0}?{0}?{0}", l[0]).as_bytes().to_vec();
            let nums = format!("{0},{0},{0},{0},{0}", l[1]).split(',').map(|n| n.parse::<usize>().unwrap()).collect_vec();
            a.push((springs, nums));
            a
        });
    day12(lines).to_string()
}
