use crate::days::Day;

pub struct Day004;

impl Day for Day004 {
    fn problem(&self) -> &'static str {
        include_str!("problem.txt")
    }
    fn input(&self) -> &'static str {
        include_str!("input.txt")
    }
    fn part1(&self, input: &String) -> Result<String, String> {
        let mut result: usize = 0;

        let mut rows = input.lines();

        loop {
            match rows.next() {
                Some(r) => {
                    let (_, l) = r.split_once(": ").unwrap();
                    let (win, mine) = l.split_once(" | ").unwrap();
                    let win = win
                        .split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();
                    let mine = mine
                        .split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .fold(0, |acc, n| if win.contains(&n) { acc + 1 } else { acc });
                    let score = if mine == 0 { 0 } else { 1 << (mine - 1) };
                    result += score;
                }
                None => break,
            }
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &String) -> Result<String, String> {
        let mut result: usize = 0;

        let mut rows = input.lines();

        loop {
            let mut i: usize = 0;
            match rows.next() {
                Some(r) => {
                    let (_, l) = r.split_once(": ").unwrap();
                    let (win, mine) = l.split_once(" | ").unwrap();
                    let win = win
                        .split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();
                    let mine = mine
                        .split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .fold(0, |acc, n| if win.contains(&n) { acc + 1 } else { acc });
                    let score = if mine == 0 { 0 } else { 1 << (mine - 1) };
                    if score != 0 {}
                    i += 1;
                }
                None => break,
            }
        }

        Ok(result.to_string())
    }
}
