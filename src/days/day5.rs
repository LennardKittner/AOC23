
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

fn lookup2(seeds: &mut Vec<Seed>, dict: &Vec<Entry>) -> Vec<Seed>{
    let mut result: Vec<Seed> = Vec::new();
    let mut i = 0;
    while i < seeds.len() {
        let seed = seeds[i].clone();
        let mut found = false;
        for entry in dict {
            if entry.src <= seed.start && seed.start < entry.src + entry.len {
                let offset = seed.start - entry.src;
                if entry.len >= seed.len { 
                    result.push(Seed { start: entry.target + offset, len: seed.len });
                    println!("UPDATED SEED 1 {:?} FROM {:?} WITH {:?}",Seed { start: entry.target + offset, len: entry.len }, seed, entry);
                } else {
                    result.push(Seed { start: entry.target + offset, len: entry.len });
                    println!("UPDATED SEED 2 {:?} FROM {:?} WITH {:?}",Seed { start: entry.target + offset, len: entry.len }, seed, entry);
                    seeds.push(Seed { start: seed.start + entry.len, len: seed.len - entry.len });
                    println!("NEW SEED {:?} FROM {:?}",Seed { start: seed.start + entry.len, len: seed.len - entry.len }, seed);
                }
                found = true;
                break;
            }
        }
        if !found {
            result.push(Seed { start: seed.start, len: seed.len });
            println!("NEW SEED {:?} NO MATCH", seed)
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

    println!("SEEDS: {:?}", seeds);
    let mut tmp = lookup2(&mut seeds, &dict_s_s);
    println!("TMP: {:?}", tmp);
    let mut tmp = lookup2(&mut tmp, &dict_s_f);
    println!("TMP: {:?}", tmp);
    let mut tmp = lookup2(&mut tmp, &dict_f_w);
    println!("TMP: {:?}", tmp);
    let mut tmp = lookup2(&mut tmp, &dict_w_l);
    println!("TMP: {:?}", tmp);
    let mut tmp = lookup2(&mut tmp, &dict_l_t);
    println!("TMP: {:?}", tmp);
    let mut tmp = lookup2(&mut tmp, &dict_t_h);
    println!("TMP: {:?}", tmp);
    let result = lookup2(&mut tmp, &dict_h_l);
    println!("RESULT: {:?}", result);
    result.iter().map(|s| s.start).min().unwrap().to_string()
}
