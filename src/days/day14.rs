use itertools::Itertools;

pub fn exec_day14_part1(input: &str) -> String {
    let mut grid = input.lines().map(|c| c.as_bytes().to_vec()).collect_vec();
    let mut result = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == b'O' {
                for i in (0..y).rev() {
                    if grid[i][x] != b'.' {
                        grid[i+1][x] = b'O';
                        grid[y][x] = b'.';
                        result += grid.len() - y;
                        break;
                    }
                }
            }
        }
    }
    println!("{}", grid.iter().map(|l| l.iter().map(|c| *c as char).join("")).join("\n"));
    result.to_string()
}

pub fn exec_day14_part2(input: &str) -> String {
    todo!("{input}")
}
