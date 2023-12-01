use std::fs;

fn day1_1() {
    let contents =
        fs::read_to_string("puzzle_1_1").expect("Should have been able to read the file");
    println!("{contents}");
}

fn main() {
    println!("Hello, world!");
    day1_1();
}
