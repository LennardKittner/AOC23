use itertools::Itertools;

pub fn exec_day9_part1(input: &str) -> String {
    let mut sequences = make_sequences(input);
    for sequence in &mut sequences {
        for i in (0..sequence.len()).skip(1).rev() {
            let last_curr = *sequence[i].last().unwrap();
            let last_parent = *sequence[i-1].last().unwrap();
            sequence[i-1].push(last_parent + last_curr)
        }
    }
    sequences.iter().map(|s| s[0].last().unwrap()).sum::<i32>().to_string()
}

pub fn exec_day9_part2(input: &str) -> String {
    let mut sequences = make_sequences(input);
    for sequence in &mut sequences {
        for i in (0..sequence.len()).skip(1).rev() {
            let first_curr = *sequence[i].first().unwrap();
            let first_parent = *sequence[i-1].first().unwrap();
            sequence[i-1].insert(0 , first_parent - first_curr)
        }
    }
    sequences.iter().map(|s| s[0].first().unwrap()).sum::<i32>().to_string()
}

fn make_sequences(input: &str) -> Vec<Vec<Vec<i32>>> {
    let mut sequences = input.lines().map(|l| vec![l.split(' ').map(|v| v.parse::<i32>().unwrap()).collect_vec()]).collect_vec();
    for sequence in sequences.iter_mut() {
        let mut zeros;
        let mut i = 0;
        while i < sequence.len() {
            zeros = true;
            sequence.push(sequence[i].iter().enumerate().skip(1).fold(Vec::new(), |mut acc, v| {
                let tmp = v.1 - sequence[i][v.0 - 1];
                if tmp != 0 {
                    zeros = false;
                }
                acc.push(tmp);
                acc
            }));
            if zeros {
                break;
            }
            i += 1;
        }
    }
    sequences
}