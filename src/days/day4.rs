
pub fn exec_day4_part1(input: &str) -> String {
    let cards: Vec<Vec<Vec<u32>>> = input.replace("  ", " ").lines()
    .map(|l| l.split(": ").last().unwrap()
        .split(" | ")
        .map(|c| c.split(' ')
            .map(|n| n.parse::<u32>().unwrap()).collect()).collect()).collect();
    let mut result = 0;
    for card in cards {
        let mut match_count = -1;
        for num in &card[1] {
            if card[0].contains(num) {
                match_count += 1;
            }
        }
        if match_count >= 0 { result += 2_u32.pow(match_count as u32); }
    }

    format!("{:?}", result)
}


pub fn exec_day4_part2(input: &str) -> String {
    let mut cards: Vec<(i32, Vec<Vec<u32>>)> = input.replace("  ", " ").lines()
    .map(|l| l.split(": ").last().unwrap()
        .split(" | ")
        .map(|c| c.split(' ')
            .map(|n| n.parse::<u32>().unwrap()).collect()).collect())
        .map(|c| (1, c)).collect();
    for i in 0..cards.len() {
        let mut match_count = 0;
        for num in &cards[i].1[1] {
            if cards[i].1[0].contains(num) {
                match_count += 1;
            }
        }
        if match_count > 0 {
            for j in 1..=match_count {
                cards[i + j].0 += cards[i].0;
            }
        }
    }

    format!("{}", cards.iter().map(|c| c.0).sum::<i32>())
}