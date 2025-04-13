use core::panic;
use std::{cmp::Ordering, collections::HashMap};

use crate::days::Day;

pub struct Day008;

impl Day for Day008 {
    fn problem(&self) -> &'static str {
        include_str!("problem.txt")
    }
    fn input(&self) -> &'static str {
        include_str!("input.txt")
    }
    fn part1<'a>(&self, input: &'a String) -> Result<String, String> {
        let mut result: usize = 0;

        let mut elems: HashMap<&str, (&str, &str)> = HashMap::new();
        let mut iter = input.lines();

        let directions = iter.next().unwrap().chars().collect::<Vec<char>>();
        iter.next();

        let mut position: &str = "AAA";

        loop {
            match iter.next() {
                Some(elem) => {
                    let (k, v) = elem.split_once(" = ").unwrap();
                    let (v1, v2) = v.split_once(", ").unwrap();
                    elems.insert(
                        k,
                        (v1.strip_prefix("(").unwrap(), v2.strip_suffix(")").unwrap()),
                    );
                }
                None => break,
            }
        }
        // println!("{:?}", elems);

        let mut steps: usize = 0;

        'o: loop {
            let mut i = 0;

            while i < directions.len() {
                // println!("{}", position);
                let now = elems.get(position).unwrap();
                if position == "ZZZ" {
                    break 'o;
                }
                match directions[i] {
                    'L' => {
                        position = now.0;
                    }
                    'R' => {
                        position = now.1;
                    }
                    _ => unreachable!(),
                }
                steps += 1;
                i += 1;
            }
        }

        result = steps;
        Ok(result.to_string())
    }

    fn part2(&self, input: &String) -> Result<String, String> {
        let mut result: usize = 0;

        let mut elems: HashMap<&str, (&str, &str, usize)> = HashMap::new();
        let mut iter = input.lines();

        let directions = iter.next().unwrap().chars().collect::<Vec<char>>();
        iter.next();

        let mut positions: Vec<&str> = vec![];

        loop {
            match iter.next() {
                Some(elem) => {
                    let (k, v) = elem.split_once(" = ").unwrap();
                    let (v1, v2) = v.split_once(", ").unwrap();
                    elems.insert(
                        k,
                        (
                            v1.strip_prefix("(").unwrap(),
                            v2.strip_suffix(")").unwrap(),
                            0,
                        ),
                    );
                }
                None => break,
            }
        }
        // 0 not explored
        // 1 explored
        // 2 bad
        // println!("{:?}", elems);
        //
        for e in &elems {
            if e.0.chars().nth(2).unwrap() == 'A' {
                positions.push(*e.0);
            }
        }

        let mut steps: usize = 0;
        'o: loop {
            println!("{:?}", positions);
            let mut i = 0;

            while i < directions.len() {
                // println!("{}", position);
                for position in &mut positions {
                    let now = elems.get(position).unwrap();
                    {}
                    match directions[i] {
                        'L' => {
                            *position = now.0;
                        }
                        'R' => {
                            *position = now.1;
                        }
                        _ => unreachable!(),
                    }
                }
                steps += 1;
                i += 1;
                let mut won = true;
                for position in &positions {
                    if position.chars().nth(2).unwrap() != 'Z' {
                        won = false;
                        break;
                    }
                }
                if won {
                    break 'o;
                }
            }
        }

        result = steps;
        Ok(result.to_string())
    }
}
