use std::{collections::HashMap, fs};

use crate::puzzle::AocPuzzle;

pub fn day_4(puzzle: AocPuzzle) -> u32 {
    let puzzle_file = get_puzzle();
    match puzzle {
        AocPuzzle::PartOne => part_1(&puzzle_file),
        AocPuzzle::PartTwo => part_2(&puzzle_file),
    }
}

trait ParseVec {
    fn parse_vec(&self) -> Vec<u8>;
}

impl ParseVec for &str {
    fn parse_vec(&self) -> Vec<u8> {
        self.trim().split(' ').flat_map(|x| x.parse()).collect()
    }
}

#[derive(PartialEq, Debug)]
struct Card {
    id: u8,
    winning: Vec<u8>,
    numbers: Vec<u8>,
}

impl Card {
    fn is_winning_number(&self, n: &u8) -> bool {
        self.winning.contains(n)
    }

    fn get_winning_tickets(&self) -> Vec<&u8> {
        self.numbers
            .iter()
            .filter(|n| self.is_winning_number(n))
            .collect()
    }

    fn get_card_score(&self) -> u32 {
        match self.get_winning_tickets().len() {
            0 => 0,
            n => 2_u32.pow((n as u32) - 1),
        }
    }

    fn from_raw_line(line: &str) -> Card {
        let (left_part, right_part) = line.split_once('|').unwrap();
        let (raw_card, raw_winning) = left_part.split_once(':').unwrap();
        let id: u8 = raw_card.split_once(' ').unwrap().1.trim().parse().unwrap();

        Card {
            id,
            numbers: right_part.parse_vec(),
            winning: raw_winning.parse_vec(),
        }
    }
}

fn get_puzzle() -> String {
    fs::read_to_string("puzzle_4_1").expect("Should have been able to read the file")
}

struct CardsRegistry {
    cards_indexes: HashMap<u8, u32>,
}

impl CardsRegistry {
    fn add_index(&mut self, index: u8) {
        self.cards_indexes
            .entry(index)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
    fn new() -> CardsRegistry {
        CardsRegistry {
            cards_indexes: HashMap::new(),
        }
    }
    fn get_index(&self, index: u8) -> u32 {
        *self.cards_indexes.get(&index).unwrap_or(&0)
    }

    fn get_count(&self) -> u32 {
        self.cards_indexes.values().sum::<u32>()
    }
}

fn part_2(input: &str) -> u32 {
    let mut registry = CardsRegistry::new();
    for line in input.lines() {
        let card = Card::from_raw_line(line);
        let num_of_copies = registry.get_index(card.id);
        let winning = card.get_winning_tickets().iter().count() as u8;
        for _ in 0..=num_of_copies {
            for c in card.id + 1..=card.id + winning {
                registry.add_index(c)
            }
        }
    }
    registry.get_count() + input.lines().count() as u32
}

fn part_1(input: &str) -> u32 {
    input.lines().fold(0, |acc, val| {
        acc + Card::from_raw_line(val).get_card_score()
    })
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::day4::{part_1, part_2};

    use super::Card;

    #[test]
    fn test_parse() {
        let card1 = Card::from_raw_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(
            card1,
            Card {
                id: 1,
                winning: vec![41, 48, 83, 86, 17],
                numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            }
        );

        let card2 = Card::from_raw_line("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1");
        assert_eq!(
            card2,
            Card {
                id: 3,
                winning: vec![1, 21, 53, 59, 44],
                numbers: vec![69, 82, 63, 72, 16, 21, 14, 1]
            }
        )
    }

    #[test]
    fn test_sum() {
        let card = Card {
            id: 1,
            winning: vec![41, 48, 83, 86, 17],
            numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };

        assert_eq!(card.get_card_score(), 8);

        let card = Card {
            id: 3,
            winning: vec![1, 21, 53, 59, 44],
            numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
        };

        assert_eq!(card.get_card_score(), 2);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            13
        )
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            30
        )
    }
}
