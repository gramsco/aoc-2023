use std::env;

use puzzle::Puzzle;
mod day1;
mod day2;

mod puzzle;

fn run_day(day: &str, puzzle: puzzle::Puzzle) -> u32 {
    match day {
        "day1" => day1::day_1(puzzle),
        "day2" => day2::day_2(),
        _ => panic!("Sorry"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let x = &args[2];
    let puzzle = match x.as_str() {
        "part1" => Puzzle::First,
        "part2" => Puzzle::Second,
        _ => panic!(""),
    };

    let res = run_day(day, puzzle);
    println!("{res}");
}
