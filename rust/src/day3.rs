use std::fs;

use crate::puzzle::AocPuzzle;

#[derive(PartialEq, Debug)]
struct Num {
    value: u32,
    length: usize,
    coordinates: (usize, usize),
}
pub fn day_3(puzzle: AocPuzzle) -> u32 {
    let puzzle_file = get_puzzle();
    match puzzle {
        AocPuzzle::PartOne => part_1(&puzzle_file),
        AocPuzzle::PartTwo => unimplemented!(),
    }
}

fn get_puzzle() -> String {
    fs::read_to_string("puzzle_3_1").expect("Should have been able to read the file")
}

pub trait Adjacent {
    fn is_adjacent(&self, coords: (usize, usize)) -> bool;
}

impl Adjacent for (usize, usize) {
    fn is_adjacent(&self, coords: (usize, usize)) -> bool {
        self.0.abs_diff(coords.0) < 2 && self.1.abs_diff(coords.1) < 2
    }
}

impl Num {
    fn expands_coordinates(&self) -> Vec<(usize, usize)> {
        (self.coordinates.1..(self.coordinates.1 + self.length))
            .into_iter()
            .map(|i| (self.coordinates.0, i))
            .collect()
    }

    fn coordinates_compatible(&self, coords: (usize, usize)) -> bool {
        self.expands_coordinates()
            .iter()
            .any(|x| x.is_adjacent(coords))
    }
}

#[derive(PartialEq, Debug)]
enum Token {
    Number(Num),
    Symbol(char, (usize, usize)),
    Period,
}

fn tokenize_lines(line: &str) -> Vec<Token> {
    let mut acc = String::new();
    let mut tokens: Vec<Token> = Vec::new();
    for (line_index, line) in line.lines().enumerate() {
        for (column, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                acc.push(c);

                if column != line.len() - 1 {
                    continue;
                }
            }

            let coeff = match c.is_digit(10) {
                true => 1,
                false => 0,
            };

            if let Ok(num) = acc.parse::<u32>() {
                let length = acc.len();
                tokens.push(Token::Number(Num {
                    value: num,
                    length: length.try_into().unwrap(),
                    coordinates: (line_index, (column - length) + coeff),
                }));
                acc.clear();
            }

            if c.is_digit(10) {
                break;
            }

            match c {
                '.' => tokens.push(Token::Period),
                symbol => tokens.push(Token::Symbol(symbol, (line_index, column))),
            }
        }
    }
    tokens
}

fn part_1(input: &str) -> u32 {
    let tokens = tokenize_lines(input);
    let symbols_positions: &Vec<_> = &tokens
        .iter()
        .filter_map(|x| match x {
            Token::Symbol(_, coordinates) => Some(coordinates),
            _ => None,
        })
        .collect();

    let mut sum = 0;
    for token in &tokens {
        match token {
            Token::Number(n) => {
                for s in symbols_positions.clone() {
                    if n.coordinates_compatible(*s) {
                        sum = sum + n.value
                    }
                }
            }
            _ => continue,
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::day3::{part_1, tokenize_lines, Num, Token};

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            4361
        )
    }

    #[test]
    fn test_expand_coordinates() {
        let n = Num {
            value: 454,
            coordinates: (5, 6),
            length: 4,
        };
        assert_eq!(
            n.expands_coordinates(),
            vec![(5, 6), (5, 7), (5, 8), (5, 9)]
        )
    }
    #[test]
    fn test_adjacency() {
        let n = Num {
            value: 12,
            coordinates: (5, 1),
            length: 2,
        };

        // above
        assert!(n.coordinates_compatible((4, 0)));
        assert!(n.coordinates_compatible((4, 1)));
        assert!(n.coordinates_compatible((4, 2)));
        assert!(n.coordinates_compatible((4, 3)));
        assert!(!n.coordinates_compatible((4, 4)));

        // same line
        assert!(n.coordinates_compatible((5, 0)));
        assert!(n.coordinates_compatible((5, 2)));
        assert!(n.coordinates_compatible((5, 3)));
        assert!(!n.coordinates_compatible((5, 4)));

        // under
        assert!(n.coordinates_compatible((5, 0)));
        assert!(n.coordinates_compatible((5, 1)));
        assert!(n.coordinates_compatible((5, 2)));
        assert!(n.coordinates_compatible((5, 3)));
        assert!(!n.coordinates_compatible((5, 4)));
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            tokenize_lines(".867"),
            vec![
                Token::Period,
                Token::Number(Num {
                    value: 867,
                    coordinates: (0, 1),
                    length: 3
                }),
            ]
        );
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
