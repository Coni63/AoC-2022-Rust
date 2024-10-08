use itertools::Itertools;
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

fn parse_input(filename: &str) -> (Vec<VecDeque<char>>, Vec<(usize, usize, usize)>) {
    let mut stacks: Vec<VecDeque<char>> = (0..10).map(|_| VecDeque::new()).collect();
    let mut actions: Vec<(usize, usize, usize)> = Vec::new();

    let mut part1 = true;
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            if part1 {
                let mut idx = 0;
                for chunk in &line.chars().skip(1).chunks(4) {
                    if let Some(first_char) = chunk.into_iter().next() {
                        if first_char == '1' {
                            part1 = false;
                            break;
                        } else if first_char != ' ' {
                            stacks[idx].push_front(first_char);
                        }
                        idx += 1;
                    }
                }
            } else {
                if line.is_empty() {
                    continue;
                }

                let parts: Vec<&str> = line.split_whitespace().collect();

                let quantity: usize = parts[1].parse().unwrap();
                let from: usize = parts[3].parse().unwrap();
                let to: usize = parts[5].parse().unwrap();

                actions.push((quantity, from - 1, to - 1));
            }
        }
    }
    (stacks, actions)
}

fn solve_part_1(mut stacks: Vec<VecDeque<char>>, actions: &[(usize, usize, usize)]) -> String {
    let mut ans = String::new();

    for (quantity, from, to) in actions.iter() {
        for _ in 0..*quantity {
            if let Some(l) = stacks[*from].pop_back() {
                stacks[*to].push_back(l);
            }
        }
        // eprintln!("{:?}", stacks);
    }

    for queue in stacks.iter_mut() {
        if let Some(l) = queue.pop_back() {
            ans.push(l);
        }
    }

    ans
}

fn solve_part_2(mut stacks: Vec<VecDeque<char>>, actions: &[(usize, usize, usize)]) -> String {
    let mut ans = String::new();

    for (quantity, from, to) in actions.iter() {
        let mut tmp: VecDeque<char> = VecDeque::new();
        for _ in 0..*quantity {
            if let Some(l) = stacks[*from].pop_back() {
                tmp.push_back(l);
            }
        }

        while let Some(l) = tmp.pop_back() {
            stacks[*to].push_back(l);
        }
        // eprintln!("{:?}", stacks);
    }

    for queue in stacks.iter_mut() {
        if let Some(l) = queue.pop_back() {
            ans.push(l);
        }
    }

    ans
}

pub fn run() {
    let (stacks, actions) = parse_input("src/inputs/day5.txt");
    // eprintln!("{:?}", stacks);
    // eprintln!("{:?}", actions);

    let p1 = solve_part_1(stacks.clone(), &actions);
    let p2 = solve_part_2(stacks.clone(), &actions);

    println!("Solution for Day 2 - part 1: {}", p1);
    println!("Solution for Day 2 - part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_priority() {}
}
