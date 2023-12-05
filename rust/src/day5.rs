use std::fs;

use crate::puzzle::AocPuzzle;

pub fn day_5(puzzle: AocPuzzle) -> u64 {
    let puzzle_file = get_puzzle();
    match puzzle {
        AocPuzzle::PartOne => part_1(&puzzle_file),
        AocPuzzle::PartTwo => part_2(&puzzle_file),
    }
}

fn get_puzzle() -> String {
    fs::read_to_string("puzzle_5_1").expect("Should have been able to read the file")
}

fn part_1(input: &str) -> u64 {
    Almanac::from_raw_str(input).run()
}

fn part_2(input: &str) -> u64 {
    unimplemented!()
}

#[derive(PartialEq, Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Almanac {
    fn from_raw_str(input: &str) -> Almanac {
        let mut a = Almanac {
            seeds: vec![],
            maps: vec![],
        };

        let mut fns_acc: Vec<Fn> = vec![];

        for line in input.lines() {
            if line.starts_with("seeds") {
                a.seeds = line.split_whitespace().flat_map(|x| x.parse()).collect();
                continue;
            }

            if line.contains("map:") {
                if &fns_acc.len() > &0 {
                    a.maps.push(Map {
                        // dirty
                        fns: fns_acc.clone(),
                    });
                }
                fns_acc.clear();
                continue;
            }

            let nums: Vec<u64> = line.split_whitespace().flat_map(|x| x.parse()).collect();
            if nums.len() > 0 {
                let to = nums.get(0).expect("Weird 0");
                let from = nums.get(1).expect("Weird 1");
                let size = nums.get(2).expect("Weird 2");
                fns_acc.push(Fn::from(*to, *from, *size));
            }
        }

        a.maps.push(Map {
            fns: fns_acc.clone(),
        });

        a
    }
    fn run(&self) -> u64 {
        self.seeds
            .iter()
            .map(|s| {
                let r = self.maps.iter().fold(*s, |acc, m| m.get(acc));
                r
            })
            .min()
            .unwrap()
    }
}

#[derive(PartialEq, Debug)]
struct Map {
    fns: Vec<Fn>,
}

impl Map {
    fn get(&self, n: u64) -> u64 {
        self.fns.iter().find_map(|v| v.get(n)).unwrap_or(n)
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Fn {
    from: u64,
    to: u64,
    size: u64,
}

impl Fn {
    fn contains(&self, n: u64) -> bool {
        n >= self.from && n < self.from + self.size
    }
    fn get(&self, n: u64) -> Option<u64> {
        if self.contains(n) {
            let index = n - self.from;
            return Some(self.to + index);
        }
        None
    }

    fn from(to: u64, from: u64, size: u64) -> Fn {
        Fn { from, to, size }
    }
}

#[cfg(test)]
mod tests {
    use super::{Almanac, Fn, Map};

    #[test]
    fn part_1() {
        assert_eq!(
            Almanac::from_raw_str(
                "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4"
            )
            .run(),
            35
        );
    }

    #[test]
    fn parse_input() {
        assert_eq!(
            Almanac::from_raw_str(
                "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4"
            ),
            Almanac {
                seeds: vec![79, 14, 55, 13],
                maps: vec![
                    // Seed to soil
                    Map {
                        fns: vec![Fn::from(50, 98, 2), Fn::from(52, 50, 48)],
                    },
                    // Soil to fertilizer
                    Map {
                        fns: vec![
                            Fn::from(0, 15, 37),
                            Fn::from(37, 52, 2),
                            Fn::from(39, 0, 15)
                        ],
                    },
                    // Fertilizer-to-water
                    Map {
                        fns: vec![
                            Fn::from(49, 53, 8),
                            Fn::from(0, 11, 42),
                            Fn::from(42, 0, 7),
                            Fn::from(57, 7, 4)
                        ]
                    },
                    // Water to light
                    Map {
                        fns: vec![Fn::from(88, 18, 7), Fn::from(18, 25, 70)]
                    },
                    // Light to temperature
                    Map {
                        fns: vec![
                            Fn::from(45, 77, 23),
                            Fn::from(81, 45, 19),
                            Fn::from(68, 64, 13)
                        ]
                    },
                    // temperatuer to humidity
                    Map {
                        fns: vec![Fn::from(0, 69, 1), Fn::from(1, 0, 69)]
                    },
                    // humidity to location,
                    Map {
                        fns: vec![Fn::from(60, 56, 37), Fn::from(56, 93, 4)]
                    }
                ],
            }
        )
    }

    #[test]
    fn test_contains() {
        let f = Fn {
            from: 98,
            to: 50,
            size: 2,
        };

        assert_eq!(f.get(98), Some(50));
        assert_eq!(f.get(99), Some(51));
        assert_eq!(f.get(100), None)
    }

    #[test]
    fn test_get_map() {
        let m1 = Map {
            fns: vec![
                Fn {
                    from: 98,
                    to: 50,
                    size: 2,
                },
                Fn {
                    from: 50,
                    to: 52,
                    size: 48,
                },
            ],
        };

        assert_eq!(m1.get(79), 81);
        assert_eq!(m1.get(14), 14);
        assert_eq!(m1.get(55), 57);
        assert_eq!(m1.get(13), 13);
    }

    #[test]
    fn test_run_almanac() {
        let a = Almanac {
            seeds: vec![79, 14, 55, 13],
            maps: vec![
                Map {
                    fns: vec![
                        Fn {
                            from: 98,
                            to: 50,
                            size: 2,
                        },
                        Fn {
                            from: 50,
                            to: 52,
                            size: 48,
                        },
                    ],
                },
                Map {
                    fns: vec![
                        Fn {
                            from: 15,
                            to: 0,
                            size: 37,
                        },
                        Fn {
                            from: 52,
                            to: 37,
                            size: 2,
                        },
                        Fn {
                            from: 0,
                            to: 39,
                            size: 15,
                        },
                    ],
                },
            ],
        };
        assert_eq!(a.run(), 52);
    }
}
