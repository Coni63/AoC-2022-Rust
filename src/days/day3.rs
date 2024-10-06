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

fn parse_input(filename: &str) -> Vec<String> {
    let mut rucksacks = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for value in lines.flatten() {
            rucksacks.push(value);
        }
    }
    rucksacks
}

fn get_priority(letter: &char) -> u32 {
    match letter {
        'a'..='z' => (*letter as u32) - ('a' as u32) + 1,
        'A'..='Z' => (*letter as u32) - ('A' as u32) + 27,
        _ => 0,
    }
}

fn find_duplicated_letter(w1: &str, w2: &str) -> Result<char, ()> {
    for letter1 in w1.chars() {
        for letter2 in w2.chars() {
            if letter1 == letter2 {
                return Ok(letter1);
            }
        }
    }
    Err(())
}

fn find_common_letter(w1: &str, w2: &str, w3: &str) -> Result<char, ()> {
    let mut appearance = [0; 52];

    for word in [w1, w2, w3] {
        let mut seen = [false; 52];
        for letter in word.chars() {
            let idx = get_priority(&letter) - 1;
            if !seen[idx as usize] {
                seen[idx as usize] = true;
                appearance[idx as usize] += 1;
            }
            if appearance[idx as usize] == 3 {
                return Ok(letter);
            }
        }
    }
    Err(())
}

fn solve_part_1(rucksacks: &[String]) -> u32 {
    let mut ans = 0;
    for line in rucksacks.iter() {
        let n = line.len();
        let k = n / 2;
        let w1 = &line[0..k];
        let w2 = &line[k..n];
        if let Ok(c) = find_duplicated_letter(w1, w2) {
            ans += get_priority(&c);
        }
    }
    ans
}

fn solve_part_2(rucksacks: &[String]) -> u32 {
    let mut ans = 0;
    for chunk in rucksacks.chunks(3) {
        if let [w1, w2, w3] = chunk {
            if let Ok(c) = find_common_letter(w1, w2, w3) {
                ans += get_priority(&c);
            }
        }
    }
    ans
}

pub fn run() {
    let actions = parse_input("src/inputs/day3.txt");
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
    fn test_get_priority() {
        assert_eq!(get_priority(&'a'), 1);
        assert_eq!(get_priority(&'z'), 26);
        assert_eq!(get_priority(&'A'), 27);
        assert_eq!(get_priority(&'Z'), 52);
    }
}
