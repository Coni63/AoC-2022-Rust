use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
pub enum Action {
    Rock(u32),
    Paper(u32),
    Scissors(u32),
}

#[derive(Debug)]
pub enum Objective {
    Win,
    Lose,
    Draw,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn letter_to_action(letter: &str) -> Action {
    match letter {
        "A" => Action::Rock(1),
        "B" => Action::Paper(2),
        "C" => Action::Scissors(3),
        "X" => Action::Rock(1),
        "Y" => Action::Paper(2),
        "Z" => Action::Scissors(3),
        _ => panic!("WTF"),
    }
}

fn decide(my_action: &Action, opp_action: &Action) -> u32 {
    match (my_action, opp_action) {
        (Action::Rock(x), Action::Rock(_)) => x + 3,
        (Action::Rock(x), Action::Paper(_)) => x + 0,
        (Action::Rock(x), Action::Scissors(_)) => x + 6,
        (Action::Paper(x), Action::Rock(_)) => x + 6,
        (Action::Paper(x), Action::Paper(_)) => x + 3,
        (Action::Paper(x), Action::Scissors(_)) => x + 0,
        (Action::Scissors(x), Action::Rock(_)) => x + 0,
        (Action::Scissors(x), Action::Paper(_)) => x + 6,
        (Action::Scissors(x), Action::Scissors(_)) => x + 3,
    }
}

fn pick_action(opp_action: &Action, my_letter: &str) -> Action {
    let obj = match my_letter {
        "X" => Objective::Lose,
        "Y" => Objective::Draw,
        "Z" => Objective::Win,
        _ => panic!("WTF"),
    };

    match (opp_action, obj) {
        (Action::Rock(_), Objective::Lose) => Action::Scissors(3),
        (Action::Rock(_), Objective::Draw) => Action::Rock(1),
        (Action::Rock(_), Objective::Win) => Action::Paper(2),
        (Action::Paper(_), Objective::Lose) => Action::Rock(1),
        (Action::Paper(_), Objective::Draw) => Action::Paper(2),
        (Action::Paper(_), Objective::Win) => Action::Scissors(3),
        (Action::Scissors(_), Objective::Lose) => Action::Paper(2),
        (Action::Scissors(_), Objective::Draw) => Action::Scissors(3),
        (Action::Scissors(_), Objective::Win) => Action::Rock(1),
    }
}

fn parse_input(filename: &str) -> Vec<(String, String)> {
    let mut plays = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for value in lines.flatten() {
            let cols: Vec<&str> = value.split(" ").collect();
            plays.push((cols[0].to_owned(), cols[1].to_owned()))
        }
    }
    plays
}

fn solve_part_1(actions: &[(String, String)]) -> u32 {
    let mut score = 0;
    for (opp_action, my_action) in actions.iter() {
        score += decide(&letter_to_action(my_action), &letter_to_action(opp_action));
    }
    score
}

fn solve_part_2(actions: &[(String, String)]) -> u32 {
    let mut score = 0;
    for (opp_action, my_action) in actions.iter() {
        let opp_action = letter_to_action(opp_action);
        let my_action = pick_action(&opp_action, my_action);
        score += decide(&my_action, &opp_action);
    }
    score
}

pub fn run() {
    let actions = parse_input("src/inputs/day2.txt");
    // eprintln!("{:?}", actions);

    let p1 = solve_part_1(&actions);
    let p2 = solve_part_2(&actions);

    println!("Solution for Day 2 - part 1: {}", p1);
    println!("Solution for Day 2 - part 2: {}", p2);
}
