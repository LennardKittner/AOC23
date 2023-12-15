use itertools::Itertools;

fn hash(string: &str) -> usize {
    let mut result: usize = 0;
    for c in string.as_bytes() {
        result += *c as usize;
        result *= 17;
        result %= 256;
    }
    result
}

fn hash_2(string: &str) -> (&str, usize, i32) {
    if string.contains('-') {
        (&string[0..string.len()-1], hash(&string[0..string.len()-1]), -1)
    } else {
        let tmp = string.split('=').collect_vec();
        (tmp[0], hash(tmp[0]), tmp[1].parse::<i32>().unwrap())
    }
}

pub fn exec_day15_part1(input: &str) -> String {
    input.split(',').map(hash).sum::<usize>().to_string()
}

pub fn exec_day15_part2(input: &str) -> String {
    let instructions = input.split(',').map(hash_2).collect_vec();
    let mut boxes: Vec<Vec<(&str, i32)>> = vec![Vec::new(); 256];
    for instruction in instructions {
        if instruction.2 < 0 {
            boxes[instruction.1].retain(|e| e.0 != instruction.0);
        } else {
            let mut found = false;
            for l in boxes[instruction.1].iter_mut() {
                if l.0 == instruction.0 {
                    found = true;
                    l.1 = instruction.2;
                    break;
                }
            }
            if !found {
                boxes[instruction.1].push((instruction.0, instruction.2));
            }
        }
    }
    let mut result = 0;
    for (i, boxx) in boxes.iter().enumerate() {
        for (j, lense) in boxx.iter().enumerate() {
            result += (i+1) * (j+1) * lense.1 as usize;
        }
    }
    result.to_string()
}
