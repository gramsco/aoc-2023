use std::fs;

enum Puzzle {
    First,
    Second,
}

fn get_puzzle() -> String {
    fs::read_to_string("puzzle_1_1").expect("Should have been able to read the file")
}

fn concat_two_digit_chars(c1: char, c2: char) -> u32 {
    let mut str = String::new();
    str.push(c1);
    str.push(c2);
    str.parse().unwrap()
}

fn add_first_and_last_digit_of_line(s: &str) -> u32 {
    let first = s.chars().find(|c| c.is_numeric());
    let last = s.chars().rev().find(|c| c.is_numeric());

    concat_two_digit_chars(first.unwrap(), last.unwrap())
}

const ENGLISH_NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_english_number(s: &str) -> Option<u32> {
    ENGLISH_NUMBERS
        .iter()
        .enumerate()
        .find(|(_, x)| s.contains(*x))
        .and_then(|(i, _)| Some(i.try_into().unwrap()))
}

fn get_first_and_last_digit_with_english_words(s: &str) -> u32 {
    let mut buffer = String::new();
    let mut nums: Vec<u32> = vec![];

    for c in s.chars() {
        buffer.push(c);
        if let Some(n) = c.to_digit(10) {
            nums.push(n);
            buffer.clear();
            continue;
        }
        if let Some(n) = get_english_number(&buffer) {
            nums.push(n);
            buffer.clear();
            buffer.push(c);
        }
    }
    let first = nums.first().unwrap();
    let last = nums.last().unwrap();
    first * 10 + last
}

fn add_first_and_last_digit_of_text_lines(s: &str, puzzle: Puzzle) -> u32 {
    s.lines().fold(0, |a, b| {
        a + match puzzle {
            Puzzle::First => add_first_and_last_digit_of_line(b),
            Puzzle::Second => get_first_and_last_digit_with_english_words(b),
        }
    })
}

fn main() {
    let puzzle = get_puzzle();
    let result_1 = add_first_and_last_digit_of_text_lines(&puzzle, Puzzle::First);
    let result_2 = add_first_and_last_digit_of_text_lines(&puzzle, Puzzle::Second);
    println!("{result_2}");
}

#[cfg(test)]
mod tests {
    use crate::{add_first_and_last_digit_of_text_lines, Puzzle};

    #[test]
    fn test_puzzle1() {
        assert_eq!(
            add_first_and_last_digit_of_text_lines(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
                crate::Puzzle::First
            ),
            142
        )
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(
            add_first_and_last_digit_of_text_lines(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
                Puzzle::Second
            ),
            281
        )
    }
}
