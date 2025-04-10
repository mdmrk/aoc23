use core::panic;
use std::{cmp::Ordering, collections::HashMap};

use crate::days::Day;

pub struct Day007;

impl Day for Day007 {
    fn problem(&self) -> &'static str {
        include_str!("problem.txt")
    }
    fn input(&self) -> &'static str {
        include_str!("input.txt")
    }
    fn part1<'a>(&self, input: &'a String) -> Result<String, String> {
        let mut result: usize = 0;
        let symbols = [
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ]
        .to_vec();

        #[derive(Debug)]
        struct Card {
            label: char,
        }

        impl Card {
            fn strength(&self) -> usize {
                match self.label {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                    _ => unreachable!(),
                }
            }

            fn cmp_symbol(&self, other: &Self) -> std::cmp::Ordering {
                if self.strength() < other.strength() {
                    Ordering::Less
                } else if self.strength() > other.strength() {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
        }

        impl Ord for Card {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.cmp_symbol(other)
            }
        }

        impl PartialOrd for Card {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl PartialEq for Card {
            fn eq(&self, other: &Self) -> bool {
                self.label == other.label
            }
        }

        impl Eq for Card {}

        #[derive(Debug)]
        struct Hand {
            hand: Vec<Card>,
            group: HashMap<char, usize>,
            bid: usize,
        }

        impl Hand {
            fn hand_type(&self) -> usize {
                let mut counts: Vec<usize> = self.group.values().cloned().collect();
                counts.sort_by(|a, b| b.cmp(a));

                if counts[0] == 5 {
                    7
                } else if counts[0] == 4 {
                    6
                } else if counts[0] == 3 && counts[1] == 2 {
                    5
                } else if counts[0] == 3 {
                    4
                } else if counts[0] == 2 && counts[1] == 2 {
                    3
                } else if counts[0] == 2 {
                    2
                } else {
                    1
                }
            }
        }

        impl Ord for Hand {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                if self.hand_type() < other.hand_type() {
                    Ordering::Less
                } else if self.hand_type() > other.hand_type() {
                    Ordering::Greater
                } else {
                    for i in 0..5 {
                        if self.hand[i] < other.hand[i] {
                            return Ordering::Less;
                        } else if self.hand[i] > other.hand[i] {
                            return Ordering::Greater;
                        }
                    }
                    return Ordering::Equal;
                }
            }
        }

        impl PartialOrd for Hand {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl PartialEq for Hand {
            fn eq(&self, other: &Self) -> bool {
                if self.hand_type() != other.hand_type() {
                    return false;
                }

                for i in 0..5 {
                    if self.hand[i] != other.hand[i] {
                        return false;
                    }
                }

                true
            }
        }

        impl Eq for Hand {}

        let mut cards = input
            .lines()
            .map(|l| {
                let (h, b) = l.split_once(" ").unwrap();

                Hand {
                    hand: h
                        .chars()
                        .map(|c| {
                            if !symbols.contains(&c) {
                                panic!("LOL");
                            }
                            Card {
                                label: c.to_owned(),
                            }
                        })
                        .collect::<Vec<Card>>(),
                    group: h.chars().fold(HashMap::new(), |mut acc, c| {
                        acc.entry(c)
                            .and_modify(|counter| *counter += 1)
                            .or_insert(1);
                        acc
                    }),
                    bid: b.parse::<usize>().unwrap(),
                }
            })
            .collect::<Vec<Hand>>();

        cards.sort();
        cards.reverse();

        // println!("{:?}", cards);

        result = cards
            .iter()
            .enumerate()
            .fold(0, |acc, (i, card)| acc + (card.bid * (i + 1)));

        Ok(result.to_string())
    }

    fn part2(&self, input: &String) -> Result<String, String> {
        let mut result: usize = 1;

        Ok(result.to_string())
    }
}
