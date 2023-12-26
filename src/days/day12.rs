use itertools::Itertools;

fn combinations(springs: &[u8], nums: &[usize]) -> i32 {
    if nums.is_empty() {
        return if springs.contains(&b'#') { 0 } else { 1 };
    } else if springs.is_empty() || nums[0] > springs.len() {
        return 0;
    }
    let mut springs = springs.to_vec();
    match springs[0] {
        b'.' => {
            combinations(&springs[1..], nums)
        },
        b'#' => {
            if !springs[..nums[0]].contains(&b'.') && (springs.len() == nums[0] || springs[nums[0]] != b'#') {
                combinations(if springs.len() == nums[0] { &[] } else { &springs[(nums[0]+1)..] }, &nums[1..])
            } else {
                0
            }
        },
        b'?' => {
            springs[0] = b'#';
            let mut result = 0;
            if !springs[..nums[0]].contains(&b'.') && (springs.len() == nums[0] || springs[nums[0]] != b'#') {
                result += combinations(if springs.len() == nums[0] { &[] } else { &springs[(nums[0]+1)..] }, &nums[1..])
            }
            springs[0] = b'.';
            result += combinations(if springs.len() == 1 { &[] } else { &springs[1..] }, nums);
            result
        },
        _ => panic!("unknons symbol")
    }
}


pub fn exec_day12_part1(input: &str) -> String {
    let lines = input.lines()
    .map(|l| l.split(' ').collect_vec())
    .fold(Vec::new(), |mut a, l| {
        let springs = l[0].as_bytes();
        let nums = l[1].split(',').map(|n| n.parse::<usize>().unwrap()).collect_vec();
        a.push((springs, nums));
        a
    });
    let mut result = 0;
    for line in lines {
        let tmp = combinations(line.0, &line.1);
        result += tmp;
    }
    result.to_string()
}

pub fn exec_day12_part2(input: &str) -> String {
    todo!("{}", input.len())
}
