use std::io::{self, stdin};

use crate::days::Day;
pub struct Day014;
static ROUND_ROCK: char = 'O';
static CUBE_ROCK: char = '#';
static EMPTY: char = '.';

impl Day for Day014 {
    fn problem(&self) -> &'static str {
        include_str!("problem.txt")
    }
    fn input(&self) -> &'static str {
        include_str!("input.txt")
    }
    fn part1(&self, input: &String) -> Result<String, String> {
        let mut result: usize = 0;

        let mut map: Vec<Vec<_>> = input
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect();
        for i in &map {
            println!("{:?}", i);
        }
        println!();

        let cols = map[0].len();

        for c in 0..cols {
            'o: loop {
                let mut i: usize = 0;

                'i: while i < map.len() {
                    if map[i][c] == ROUND_ROCK {
                        if i > 0 {
                            let j = i - 1;

                            if map[j][c] == EMPTY {
                                map[j][c] = ROUND_ROCK;
                                map[i][c] = EMPTY;
                                break 'i;
                            }
                        }
                    }
                    // let mut buf = String::new();
                    // println!("i: {} j: {}", i, i);
                    // stdin().read_line(&mut buf);
                    i += 1;
                }
                if i == map.len() {
                    break 'o;
                }
            }
        }
        for (i, line) in map.iter().enumerate() {
            for c in line {
                if *c == ROUND_ROCK {
                    result += map.len() - i;
                }
            }
        }

        Ok(result.to_string())
    }
    fn part2(&self, input: &String) -> Result<String, String> {
        let result: usize = 0;
        Ok(result.to_string())
    }
}
