mod days;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run <day>");
        return;
    }

    let day = &args[1];
    match day.as_str() {
        "1" => days::day1::run(),
        "2" => days::day2::run(),
        "3" => days::day3::run(),
        "4" => days::day4::run(),
        _ => eprintln!("Invalid day: {}. Please enter a day between 1 and 25.", day),
    }
}
