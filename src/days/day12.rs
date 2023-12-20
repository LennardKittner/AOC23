use itertools::Itertools;

fn combinations(springs: &[u8], nums: &[usize]) -> i32 {
    if nums.is_empty() {
        return 1;
    } else if springs.is_empty() {
        return 0;
    } else if nums[0] > springs.len() {
        return 0;
    }
    let mut springs = springs.to_vec();
    match springs[0] {
        b'.' => {
            combinations(&springs[1..springs.len()], &nums)
        },
        b'?' => {
            springs[0] = b'#';
            let mut result = 0;
            if !springs[0..nums[0]].contains(&b'.') && (springs.len() == nums[0] ||  springs[nums[0]] == b'.') {
                result += combinations(&springs[(nums[0]+1)..springs.len()], &nums[1..nums.len()]);
            }
            springs[0] = b'.';
            result += combinations(&springs[1..springs.len()], &nums);
            result
        }
        b'#' => {
            if !springs[0..nums[0]].contains(&b'.') && springs[nums[0]+1] == b'.' {
                combinations(&springs[(nums[0]+1)..springs.len()], &nums[1..nums.len()])
            } else {
                0
            }
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
        println!("{:?}", tmp);
        //println!("{:?}", v);
    }
    result.to_string()
}

pub fn exec_day12_part2(input: &str) -> String {
    1.to_string()
}
