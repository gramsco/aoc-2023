use std::fs;

use crate::puzzle::AocPuzzle;

#[derive(Debug, PartialEq)]
struct Set {
    r: u32,
    g: u32,
    b: u32,
}

pub fn day_2(puzzle: AocPuzzle) -> u32 {
    let puzzle_lines = get_puzzle();
    match puzzle {
        AocPuzzle::PartOne => puzzle_1(&puzzle_lines),
        AocPuzzle::PartTwo => puzzle_2(&puzzle_lines),
    }
}

fn get_puzzle() -> String {
    fs::read_to_string("puzzle_2_1").expect("Should have been able to read the file")
}

impl Set {
    fn contains(&self, set: &Set) -> bool {
        self.r >= set.r && self.g >= set.g && self.b >= set.b
    }

    fn power(&self) -> u32 {
        self.r * self.g * self.b
    }

    fn make(r: u32, g: u32, b: u32) -> Set {
        Set { r, g, b }
    }

    fn from_raw_str(s: &str) -> Set {
        s.trim()
            .split(',')
            .fold(Set { b: 0, g: 0, r: 0 }, |mut acc, val| {
                let s: Vec<_> = val.trim().split(' ').collect();
                let value: u32 = s[0].parse().unwrap();
                let color = s[1];
                match color {
                    "red" => acc.r = value,
                    "green" => acc.g = value,
                    "blue" => acc.b = value,
                    _ => panic!("ILLEGAL COLOR {color}"),
                }
                acc
            })
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn can_be_set(&self, ref_set: Set) -> bool {
        self.sets.iter().all(|set| ref_set.contains(&set)) // 12,14,13
    }

    fn get_minimum_set_of_cubes(&self) -> Set {
        self.sets.iter().fold(Set::make(0, 0, 0), |acc, set| Set {
            r: acc.r.max(set.r),
            g: acc.g.max(set.g),
            b: acc.b.max(set.b),
        })
    }

    fn from_raw_str(s: &str) -> Game {
        let raw_game_str: Vec<_> = s.split(':').collect();
        let game_id = {
            let a: Vec<_> = raw_game_str[0].split(' ').collect();
            a[1].parse().unwrap()
        };

        let sets = raw_game_str[1]
            .split(';')
            .map(|set| Set::from_raw_str(set))
            .collect();

        Game { id: game_id, sets }
    }
}

fn puzzle_1(puzzle: &str) -> u32 {
    puzzle
        .lines()
        .map(|l| Game::from_raw_str(l))
        .fold(0, |acc, val| {
            acc + match val.can_be_set(Set::make(12, 13, 14)) {
                true => val.id,
                false => 0,
            }
        })
}

fn puzzle_2(puzzle: &str) -> u32 {
    puzzle
        .lines()
        .map(|l| Game::from_raw_str(l))
        .fold(0, |acc, val| acc + val.get_minimum_set_of_cubes().power())
}

#[cfg(test)]
mod tests {
    use crate::day2::{puzzle_1, puzzle_2, Game, Set};

    #[test]
    fn test_puzzle_2() {
        assert_eq!(
            puzzle_2(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            2286
        );
    }

    #[test]
    fn test_minimal_set_of_cubes() {
        let game = Game::from_raw_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(game.get_minimum_set_of_cubes(), Set::make(4, 2, 6));
    }

    #[test]
    fn test_puzzle_1() {
        assert_eq!(
            puzzle_1(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            8
        );
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            Game::from_raw_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Game {
                id: 1,
                sets: vec![Set::make(4, 0, 3), Set::make(1, 2, 6), Set::make(0, 2, 0)]
            }
        );

        assert_eq!(Game::from_raw_str("Game 14: 1 red, 4 blue, 3 green; 2 red, 1 green, 1 blue; 1 red, 5 green, 1 blue; 3 red, 4 green, 4 blue
        "), Game {
            id:14,
            sets:vec![Set::make(1,3,4), Set::make(2,1,1), Set::make(1,5,1), Set::make(3,4,4)]
        });

        assert_eq!(
            Game::from_raw_str("Game 100: 2 blue, 1 red; 4 blue, 2 red, 1 green; 7 red, 5 blue; 2 red, 1 green, 5 blue"),
            Game {
                id: 100,
                sets: vec![Set::make(1, 0, 2), Set::make(2, 1, 4), Set::make(7, 0, 5), Set::make(2, 1, 5)]
            }
        );
    }

    #[test]
    fn test_game_is_possible() {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let game1 = Game {
            id: 1,
            sets: vec![Set::make(4, 3, 0), Set::make(1, 2, 6), Set::make(0, 2, 0)],
        };

        assert!(game1.can_be_set(Set::make(12, 13, 14)));

        // Game 3 : 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        let game3 = Game {
            id: 2,
            sets: vec![Set::make(20, 8, 6), Set::make(4, 5, 13), Set::make(1, 5, 0)],
        };

        assert!(!game3.can_be_set(Set::make(12, 13, 14)));
    }
}
