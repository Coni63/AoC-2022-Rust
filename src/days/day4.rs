use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input(filename: &str) -> Vec<(usize, usize, usize, usize)> {
    let mut ans = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for value in lines.flatten() {
            let parts: Vec<&str> = value.split(',').collect();
            let parts1: Vec<&str> = parts[0].split('-').collect();
            let parts2: Vec<&str> = parts[1].split('-').collect();
            ans.push((
                parts1[0].parse::<usize>().unwrap(),
                parts1[1].parse::<usize>().unwrap(),
                parts2[0].parse::<usize>().unwrap(),
                parts2[1].parse::<usize>().unwrap(),
            ));
        }
    }
    ans
}

fn solve_part_1(inputs: &[(usize, usize, usize, usize)]) -> u32 {
    let mut ans = 0;

    for (a, b, c, d) in inputs.iter() {
        let first_outside = (a <= c) && (b >= d);
        let first_inside = (a >= c) && (b <= d);
        if first_outside || first_inside {
            ans += 1;
        }
    }

    ans
}

fn solve_part_2(inputs: &[(usize, usize, usize, usize)]) -> u32 {
    let mut ans = 0;

    for (a, b, c, d) in inputs.iter() {
        if a <= d && b >= c {
            ans += 1;
        }
    }

    ans
}

pub fn run() {
    let actions = parse_input("src/inputs/day4.txt");
    // eprintln!("{:?}", actions);

    let p1 = solve_part_1(&actions);
    let p2 = solve_part_2(&actions);

    println!("Solution for Day 2 - part 1: {}", p1);
    println!("Solution for Day 2 - part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_priority() {}
}
