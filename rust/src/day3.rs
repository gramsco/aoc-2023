use core::panic;

use crate::puzzle::AocPuzzle;

#[derive(PartialEq, Debug)]
struct Num {
    value: u32,
    length: u8,
    coordinates: (usize, usize),
}

#[derive(PartialEq, Debug)]
enum Token {
    Number(Num),
    Symbol(char, (usize, usize)),
    Period,
}

pub fn day_3(_puzzle: AocPuzzle) -> u32 {
    unimplemented!()
}

fn tokenize_lines(line: &str) -> Vec<Token> {
    let mut acc = String::new();
    let mut tokens: Vec<Token> = Vec::new();
    for (line_index, line) in line.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                acc.push(c);
                continue;
            }

            if let Ok(num) = acc.parse::<u32>() {
                let length = acc.len();
                tokens.push(Token::Number(Num {
                    value: num,
                    length: length.try_into().unwrap(),
                    coordinates: (line_index, column - length),
                }));
                acc.clear();
            }

            match c {
                '.' => tokens.push(Token::Period),
                symbol => tokens.push(Token::Symbol(symbol, (line_index, column))),
            }
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use crate::day3::{tokenize_lines, Num, Token};

    #[test]
    fn test_find_num_with_symbols() {}

    #[test]
    fn test_parse_line() {
        assert_eq!(
            tokenize_lines(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
            ),
            vec![
                // BEGIN LINE 1
                Token::Number(Num {
                    value: 467,
                    length: 3,
                    coordinates: (0, 0)
                }),
                Token::Period,
                Token::Period,
                Token::Number(Num {
                    value: 114,
                    length: 3,
                    coordinates: (0, 5)
                }),
                Token::Period,
                Token::Period,
                // END LINE 1

                // BEGIN LINE 2
                Token::Period,
                Token::Period,
                Token::Period,
                Token::Symbol('*', (1, 3)),
                Token::Period,
                Token::Period,
                Token::Period,
                Token::Period,
                Token::Period,
                Token::Period,
                // END LINE 2

                // BEGIN LINE 3
                Token::Period,
                Token::Period,
                Token::Number(Num {
                    value: 35,
                    length: 2,
                    coordinates: (2, 2)
                }),
                Token::Period,
                Token::Period,
                Token::Number(Num {
                    value: 633,
                    length: 3,
                    coordinates: (2, 6)
                }),
                Token::Period,
                // END LINE 3

                // BEGIN LINE 4
                Token::Period,
                Token::Period,
                Token::Period,
                Token::Period,
                Token::Period,
                Token::Period,
                Token::Symbol('#', (3, 6)),
                Token::Period,
                Token::Period,
                Token::Period,
                // END LINE 4

                // BEGIN LINE 5
                Token::Number(Num {
                    value: 617,
                    length: 3,
                    coordinates: (4, 0)
                }),
                Token::Symbol('*', (4, 3)),
                Token::Period,
                Token::Period,
                Token::Period,
                Token::Period,
                Token::Period,
                Token::Period,
                // END LINE 5

                // BEGIN Line 6
                Token::Period,
                Token::Period,
                Token::Period,
                Token::Period,
                Token::Period,
                Token::Symbol('+', (5, 5)),
                Token::Period,
                Token::Number(Num {
                    value: 58,
                    coordinates: (5, 7),
                    length: 2
                }),
                Token::Period,
                // END Line 6

                // // BEGIN LINE 7
                Token::Period,
                Token::Period,
                Token::Number(Num {
                    value: 592,
                    length: 3,
                    coordinates: (6, 2)
                }),
                Token::Period,
                Token::Period,
                Token::Period,
                Token::Period,
                Token::Period,
                // END LINE 7

                // BEGIN LINE 8
                Token::Period,
                Token::Period,
                Token::Period,
                Token::Period,
                Token::Period,
                Token::Period,
                Token::Number(Num {
                    value: 755,
                    length: 3,
                    coordinates: (7, 6)
                }),
                Token::Period,
                // END LINE 8

                // BEGIN LINE 9
                Token::Period,
                Token::Period,
                Token::Period,
                Token::Symbol('$', (8, 3)),
                Token::Period,
                Token::Symbol('*', (8, 5)),
                Token::Period,
                Token::Period,
                Token::Period,
                Token::Period,
                // END LINE 9

                // BEGIN LINE 10
                Token::Period,
                Token::Number(Num {
                    value: 664,
                    length: 3,
                    coordinates: (9, 1)
                }),
                Token::Period,
                Token::Number(Num {
                    value: 598,
                    length: 3,
                    coordinates: (9, 5)
                }),
                Token::Period,
                Token::Period,
                // END LINE 10
            ]
        )
    }
}
