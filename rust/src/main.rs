use std::fs;

fn get_puzzle() -> String {
    fs::read_to_string("puzzle_1_1").expect("Should have been able to read the file")
}

fn add_first_and_last_digit_of_line(s: &str) -> u32 {
    let first = s.chars().find(|c| c.is_numeric());
    let last = s.chars().rev().find(|c| c.is_numeric());

    let mut str = String::new();
    str.push(first.unwrap());
    str.push(last.unwrap());
    str.parse().unwrap()
}

fn add_first_and_last_digit_of_text_lines(s: String) -> u32 {
    s.lines()
        .fold(0, |a, b| a + add_first_and_last_digit_of_line(b))
}

fn main() {
    let puzzle = get_puzzle();
    let result = add_first_and_last_digit_of_text_lines(puzzle);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use crate::add_first_and_last_digit_of_text_lines;

    #[test]
    fn test_puzzle1() {
        assert_eq!(
            add_first_and_last_digit_of_text_lines(String::from(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
            )),
            142
        )
    }
}
