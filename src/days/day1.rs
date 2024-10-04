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

fn parse_input(filename: &str) -> Vec<u32> {
    let mut elves: Vec<u32> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        let mut elve: u32 = 0;
        for value in lines.flatten() {
            match value.parse::<u32>() {
                Ok(x) => elve += x,
                Err(_) => {
                    elves.push(elve);
                    elve = 0;
                }
            }
        }

        if elve > 0 {
            elves.push(elve);
        }
    }

    elves
}

fn solve_part_1(elves: &[u32]) -> u32 {
    *elves.iter().max().unwrap_or(&0)
}

fn solve_part_2(elves: &[u32]) -> u32 {
    let mut copy: Vec<u32> = elves.to_vec();
    copy.sort_by(|a, b| b.cmp(a));
    copy.iter().take(3).sum()
}

pub fn run() {
    let elves = parse_input("src/inputs/day1.txt");

    // eprintln!("{:?}", elves);

    let p1 = solve_part_1(&elves);
    let p2 = solve_part_2(&elves);

    println!("Solution for Day 1 - part 1: {}", p1);
    println!("Solution for Day 1 - part 2: {}", p2);
}
