use std::ops::Range;

fn ints(string: &str) -> Vec<i64> {
    string.split(' ').filter_map(|s| s.parse::<i64>().ok()).collect()
}

#[derive(Debug)]
struct Entry {
    src: Range<i64>,
    shift: i64,
}

fn make_dict(input: &[&str], index: usize) -> Vec<Entry> {
    let mut dict: Vec<Entry> = Vec::new();
    for item in input.iter().skip(index+1) {
        let nums = ints(item);
        if nums.is_empty() {
            break;
        }
        dict.push(Entry{src: nums[1]..nums[1]+nums[2], shift: nums[0] as i64 - nums[1] as i64})
    }
    dict
}

fn lookup(seeds: &[i64], dict: &Vec<Entry>) -> Vec<i64>{
    let mut result = Vec::new();
    for seed in seeds {
        let mut found = false;
        for entry in dict {
            if entry.src.contains(seed) {
                result.push(seed + entry.shift);
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
    range: Range<i64>,
}

impl Seed {
    fn get_left(&self, entry: &Entry) -> Option<Seed> {
        if self.range.start < entry.src.start && self.range.end > entry.src.start {
            Some(Seed {range: self.range.start..entry.src.start})
        } else {
            None
        }
    }

    fn get_middle(&self, entry: &Entry) -> Option<Seed> {
        let l = self.get_left(entry).is_none();
        let r = self.get_right(entry).is_none();
        if l && r && !entry.src.contains(&self.range.start) && !entry.src.contains(&(self.range.end - 1)) {
            None
        } else {
            Some(Seed {range: (entry.shift + i64::max(self.range.start, entry.src.start))..(i64::min(self.range.end, entry.src.end) + entry.shift)})
        }
    }

    fn get_right(&self, entry: &Entry) -> Option<Seed> {
        if self.range.start < entry.src.end && self.range.end > entry.src.end {
            Some(Seed {range: entry.src.end..self.range.end})
        } else {
            None
        }
    }
}

fn lookup2(seeds: &mut Vec<Seed>, dict: &Vec<Entry>) -> Vec<Seed>{
    let mut result: Vec<Seed> = Vec::new();
    let mut i = 0;
    while i < seeds.len() {
        let seed = seeds[i].clone();
        let mut found = false;
        for entry in dict {
            let seed1 = seed.get_left(entry);
            let seed2 = seed.get_middle(entry);
            let seed3 = seed.get_right(entry);
            //println!("seed {:?} entry {:?} seed1 {:?} seed2 {:?} seed3 {:?}", seed, entry, seed1, seed2, seed3);
            if let Some(s) = seed1 {
                seeds.push(s);
            }
            if let Some(s) = seed2 {
                result.push(s);
                found = true;
            }
            if let Some(s) = seed3 {
                seeds.push(s);
            }
            if found {
                break;
            }
        }
        if !found {
            result.push(Seed { range: seed.range });
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
        seeds.push(Seed { range: seeds_values[i]..seeds_values[i]+seeds_values[i+1] });
    }

    let mut tmp = lookup2(&mut seeds, &dict_s_s);
    //println!("ss TMP {:?}", tmp);
    let mut tmp = lookup2(&mut tmp, &dict_s_f);
    // println!("sf TMP {:?}", tmp);
    let mut tmp = lookup2(&mut tmp, &dict_f_w);
    // println!("fw TMP {:?}", tmp);
    let mut tmp = lookup2(&mut tmp, &dict_w_l);
    // println!("wl TMP {:?}", tmp);
    let mut tmp = lookup2(&mut tmp, &dict_l_t);
    // println!("lt TMP {:?}", tmp);
    let mut tmp = lookup2(&mut tmp, &dict_t_h);
    // println!("th TMP {:?}", tmp);
    let result = lookup2(&mut tmp, &dict_h_l);
    // println!("hl TMP {:?}", result);
    result.iter().map(|s| s.range.start).min().unwrap().to_string()
}
