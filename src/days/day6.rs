use std::collections::VecDeque;
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

fn parse_input(filename: &str) -> String {
    if let Ok(lines) = read_lines(filename) {
        lines.flatten().collect()
    } else {
        String::new()
    }
}

fn get_first_signal(input: &str, length: usize) -> u32 {
    let mut seen: VecDeque<char> = VecDeque::new();

    for (i, c) in input.chars().enumerate() {
        if seen.contains(&c) {
            while let Some(s) = seen.pop_front() {
                if s == c {
                    break;
                }
            }
        }
        seen.push_back(c);

        if seen.len() == length {
            return (i + 1) as u32;
        }
    }

    0
}

fn solve_part_1(input: &str) -> u32 {
    get_first_signal(input, 4)
}

fn solve_part_2(input: &str) -> u32 {
    get_first_signal(input, 14)
}

pub fn run() {
    let input = parse_input("src/inputs/day6.txt");
    // eprintln!("{:?}", input);

    let p1 = solve_part_1(&input);
    let p2 = solve_part_2(&input);

    println!("Solution for Day 2 - part 1: {}", p1);
    println!("Solution for Day 2 - part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_priority() {}
}
