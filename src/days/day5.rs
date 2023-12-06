
fn ints(string: &str) -> Vec<u64> {
    string.split(" ").filter_map(|s| s.parse::<u64>().ok()).collect()
}

#[derive(Debug)]
struct Entry {
    src: u64,
    target: u64,
    len: u64,
}

fn make_dict(input: &Vec<&str>, index: usize) -> Vec<Entry> {
    let mut dict: Vec<Entry> = Vec::new();
    for i in (index+1)..input.len() {
        let nums = ints(input[i]);
        if nums.is_empty() {
            break;
        }
        dict.push(Entry{src: nums[1], target: nums[0], len: nums[2]})
    }
    dict
}

fn lookup(seeds: &Vec<u64>, dict: &Vec<Entry>) -> Vec<u64>{
    let mut result = Vec::new();
    for seed in seeds {
        let mut found = false;
        for entry in dict {
            if entry.src <= *seed && *seed < entry.src + entry.len {
                let offset = seed - entry.src;
                result.push(entry.target + offset);
                found = true;
                break;
            }
        }
        if !found {
            result.push(*seed)
        }
    }
    result
}

pub fn exec_day5_part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let seeds = ints(lines[0]);
    let index = lines.iter().position(|l| l == &"seed-to-soil map:").unwrap();
    let dict_s_s = make_dict(&lines, index);
    let index = lines.iter().position(|l| l == &"soil-to-fertilizer map:").unwrap();
    let dict_s_f = make_dict(&lines, index);
    let index = lines.iter().position(|l| l == &"fertilizer-to-water map:").unwrap();
    let dict_f_w = make_dict(&lines, index);
    let index = lines.iter().position(|l| l == &"water-to-light map:").unwrap();
    let dict_w_l = make_dict(&lines, index);
    let index = lines.iter().position(|l| l == &"light-to-temperature map:").unwrap();
    let dict_l_t = make_dict(&lines, index);
    let index = lines.iter().position(|l| l == &"temperature-to-humidity map:").unwrap();
    let dict_t_h = make_dict(&lines, index);
    let index = lines.iter().position(|l| l == &"humidity-to-location map:").unwrap();
    let dict_h_l = make_dict(&lines, index);
    
    let tmp = lookup(&seeds, &dict_s_s);
    let tmp = lookup(&tmp, &dict_s_f);
    let tmp = lookup(&tmp, &dict_f_w);
    let tmp = lookup(&tmp, &dict_w_l);
    let tmp = lookup(&tmp, &dict_l_t);
    let tmp = lookup(&tmp, &dict_t_h);
    let result = lookup(&tmp, &dict_h_l);
    result.iter().min().unwrap().to_string()
}

#[derive(Debug, Clone)]
struct Seed {
    start: u64,
    len: u64,
}

// impl Seed {
//     fn end(&self) -> u64 {
//       self.start + self.len-1  
//     }
// }

// impl Entry {
//     fn end_src(&self) -> u64 {
//       self.src + self.len-1  
//     }
// }

// fn left(a1: u64, a2: u64, b1: u64, _: u64) -> Option<(u64, u64)> {
//     if a1 >= b1 {
//         return None;
//     }
//     if a2 <= b1 {
//         return None;
//     }
//     Some((a1, b1))
// }

// fn center() {

// }

// fn right(a1: u64, a2: u64, b1: u64, b2: u64) -> Option<(u64, u64)> {
//     todo!()
// }

fn lookup2(seeds: &mut Vec<Seed>, dict: &Vec<Entry>) -> Vec<Seed>{
    let mut result: Vec<Seed> = Vec::new();
    let mut i = 0;
    while i < seeds.len() {
        let seed = seeds[i].clone();
        let mut found = false;
        for entry in dict {
            let mut seed1 = None;
            let mut seed2 = None;
            let mut seed3 = None;
            if seed.start < entry.src && seed.start + seed.len > entry.src {
                seed1 = Some(Seed { start: seed.start, len: entry.src - seed.start});
                found = true;
            }
            if seed.start < entry.src + entry.len && seed.start + seed.len >= entry.src + entry.len {
                seed3 = Some(Seed { start: entry.src + entry.len, len: seed.start + seed.len - (entry.src + entry.len)});
                found = true;
            }
            if seed.start <= entry.src && seed.start + seed.len > entry.src || seed.start < entry.src + entry.len && seed.start + seed.len > entry.src + entry.len {
                let len = if seed3.is_none() && seed1.is_none() {
                    seed.len
                } else if seed3.is_none() && seed1.is_some() {
                    seed.len - (entry.src - seed.start)
                } else if seed3.is_some() && seed1.is_none() {
                    entry.len - (seed.start - entry.src)
                } else {
                    seed.len - (entry.src - seed.start) - (seed.start + seed.len - (entry.src + entry.len))
                };
                let offset = seed.start - entry.src;
                seed2 = Some(Seed { start: entry.target + offset, len: len});
                found = true;
            }
            if found {
                println!("Seed {:?} entry {:?} seed1 {:?} seed2 {:?} seed3 {:?}", seed, entry, seed1, seed2, seed3);
            }
            if let Some(s) = seed1 {
                seeds.push(s);
            }
            if let Some(s) = seed2 {
                result.push(s);
            }
            if let Some(s) = seed3 {
                seeds.push(s);
            }
            if found {
                break;
            }

            // if seed.start < entry.src && seed.start + seed.len > entry.src {
            //     seeds.push(Seed { start: seed.start, len: entry.src - seed.start });
            //     if seed.start + seed.len <= entry.src + entry.len { 
            //         result.push(Seed { start: entry.target, len: seed.len - (entry.src - seed.start) });
            //     } else {
            //         result.push(Seed { start: entry.target, len: entry.len });
            //         seeds.push(Seed { start: entry.src + entry.len, len: seed.len - entry.len });
            //     }
            //     found = true;
            //     break;
            // } else if entry.src <= seed.start && seed.start < entry.src + entry.len {
            //     let offset = seed.start - entry.src;
            //     if entry.len >= seed.len { 
            //         result.push(Seed { start: entry.target + offset, len: seed.len });
            //     } else {
            //         result.push(Seed { start: entry.target + offset, len: entry.len });
            //         seeds.push(Seed { start: seed.start + entry.len, len: seed.len - entry.len });
            //     }
            //     found = true;
            //     break;
            // }
        }
        if !found {
            result.push(Seed { start: seed.start, len: seed.len });
        }
        i += 1;
    }
    result
}

pub fn exec_day5_part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let index = lines.iter().position(|l| l == &"seed-to-soil map:").unwrap();
    let dict_s_s = make_dict(&lines, index);
    let index = lines.iter().position(|l| l == &"soil-to-fertilizer map:").unwrap();
    let dict_s_f = make_dict(&lines, index);
    let index = lines.iter().position(|l| l == &"fertilizer-to-water map:").unwrap();
    let dict_f_w = make_dict(&lines, index);
    let index = lines.iter().position(|l| l == &"water-to-light map:").unwrap();
    let dict_w_l = make_dict(&lines, index);
    let index = lines.iter().position(|l| l == &"light-to-temperature map:").unwrap();
    let dict_l_t = make_dict(&lines, index);
    let index = lines.iter().position(|l| l == &"temperature-to-humidity map:").unwrap();
    let dict_t_h = make_dict(&lines, index);
    let index = lines.iter().position(|l| l == &"humidity-to-location map:").unwrap();
    let dict_h_l = make_dict(&lines, index);
    
    let seeds_values = ints(lines[0]);
    let mut seeds: Vec<Seed> = Vec::new();
    for i in (0..seeds_values.len()-1).step_by(2) {
        seeds.push(Seed { start: seeds_values[i], len: seeds_values[i+1] });
    }

    let mut tmp = lookup2(&mut seeds, &dict_s_s);
    println!("1");
    let mut tmp = lookup2(&mut tmp, &dict_s_f);
    println!("2");
    let mut tmp = lookup2(&mut tmp, &dict_f_w);
    println!("3");
    let mut tmp = lookup2(&mut tmp, &dict_w_l);
    println!("4");
    let mut tmp = lookup2(&mut tmp, &dict_l_t);
    println!("5");
    let mut tmp = lookup2(&mut tmp, &dict_t_h);
    println!("6");
    let result = lookup2(&mut tmp, &dict_h_l);
    println!("7");
    result.iter().map(|s| s.start).min().unwrap().to_string()
}
