use std::env;

use puzzle::AocPuzzle;
mod day1;
mod day2;
mod day3;
mod day4;

mod puzzle;

fn run_day(day: &str, puzzle: puzzle::AocPuzzle) -> u32 {
    match day {
        "day1" => day1::day_1(puzzle),
        "day2" => day2::day_2(puzzle),
        "day3" => day3::day_3(puzzle),
        "day4" => day4::day_4(puzzle),
        _ => panic!("Day not implemented yet."),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    let aoc_puzzle = match args[2].as_str() {
        "part1" => AocPuzzle::PartOne,
        "part2" => AocPuzzle::PartTwo,
        _ => panic!(""),
    };

    let res = run_day(day, aoc_puzzle);
    println!("{res}");
}
