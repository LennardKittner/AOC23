use itertools::Itertools;

fn combinations(springs: &[u8], nums: &[u64], acc: &mut Vec<String>) -> i32 {
    //println!("{:?} {:?}", springs.iter().map(|c| *c as char).collect_vec(), nums);
    if !springs.contains(&b'?') && !springs.contains(&b'#') && !nums.is_empty() {
        return 0;
    }
    if nums.is_empty() {
        // acc.push(String::new());
        println!("Exit 1");
        return 1;
    }
    let mut result = 0;
    // let mut skipped = String::new();
    for (i, c) in springs.iter().enumerate() {
        if springs[i] == b'.' {
            // skipped.push(*c as char);
            continue;
        }
        let mut valid = true;
        if i + nums[0] as usize > springs.len() {
            // skipped.push(*c as char);
            // println!("NOT HANDLED");
            return result;
        }
        for j in 0..nums[0] {
            if springs[i + j as usize] == b'.' {
                valid = false;
                break;
            }
        }
        if valid && i + nums[0] as usize + 1 >= springs.len() {
            // let before = acc.len();
            let tmp = combinations(&[], &nums[1..nums.len()], acc);
            println!("{result} + {tmp}");
            result += tmp;
            // if tmp >= 1 {
            //     for j in before..acc.len() {
            //         acc[j].insert_str(0, &skipped);
            //         for _ in 0..nums[0]-1 {
            //             acc[j].insert(i, '#');
            //         }
            //     }
            // }
        } else if valid && springs[i + nums[0] as usize] != b'#' {
            // let before: usize = acc.len();
            let tmp = combinations(&springs[(i + nums[0] as usize + 1)..springs.len()], &nums[1..nums.len()], acc);
            println!("{result} + {tmp}");
            result += tmp;
            // if tmp >= 1 {
            //     for j in before..acc.len() {
            //         acc[j].insert_str(0, &skipped);
            //         for _ in 0..nums[0] {
            //             acc[j].insert(i, '#');
            //         }
            //     }
            // }
        }
    }
    result
}


pub fn exec_day12_part1(input: &str) -> String {
    let lines = input.lines()
    .map(|l| l.split(' ').collect_vec())
    .fold(Vec::new(), |mut a, l| {
        let springs = l[0].as_bytes();
        let nums = l[1].split(',').map(|n| n.parse::<u64>().unwrap()).collect_vec();
        a.push((springs, nums));
        a
    });
    let mut result = 0;
    let mut v = Vec::new();
    for line in lines {
        let tmp = combinations(&line.0, &line.1, &mut v);
        result += tmp;
        println!("{:?}", tmp);
    }
    println!("{:?}", v);
    result.to_string()
}

pub fn exec_day12_part2(input: &str) -> String {
    todo!("{input}")
}
