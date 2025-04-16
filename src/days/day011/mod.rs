use std::{collections::HashMap, vec};

use crate::days::Day;

pub struct Day011;

static GALAXY: char = '#';
static EMPTY: char = '.';

impl Day for Day011 {
    fn problem(&self) -> &'static str {
        include_str!("problem.txt")
    }
    fn input(&self) -> &'static str {
        include_str!("input.txt")
    }
    fn part1(&self, input: &String) -> Result<String, String> {
        let mut result: usize = 0;
        let mut galaxies: Vec<(isize, isize)> = vec![];

        let mut board: Vec<Vec<char>> = input
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect();

        {
            let mut i: usize = 0;
            while i < board.len() {
                if board[i].iter().all(|c| *c == EMPTY) {
                    board.insert(i, vec![EMPTY; board[i].len()]);
                    i += 1;
                }
                i += 1;
            }
        }
        {
            let mut j: usize = 0;
            while j < board[0].len() {
                let mut ok = true;
                for i in 0..board.len() {
                    if board[i][j] != EMPTY {
                        ok = false;
                    }
                }
                if ok {
                    for i in 0..board.len() {
                        board[i].insert(j, EMPTY);
                    }
                    j += 1;
                }
                j += 1;
            }
        }
        {
            let mut i: usize = 0;
            while i < board.len() {
                let mut j: usize = 0;
                while j < board[i].len() {
                    if board[i][j] == GALAXY {
                        galaxies.push((i as isize, j as isize));
                    }
                    j += 1;
                }
                i += 1;
            }
        }

        let mut paths: usize = 0;
        for (i, g1) in galaxies.iter().enumerate() {
            for (j, g2) in galaxies.iter().enumerate() {
                let diff = ((g2.1 - g1.1).abs(), (g2.0 - g1.0).abs());
                let distance = diff.0 + diff.1;
                paths += distance as usize;
            }
        }
        result = paths / 2;
        Ok(result.to_string())
    }

    fn part2(&self, input: &String) -> Result<String, String> {
        let result: usize = 0;

        Ok(result.to_string())
    }
}
