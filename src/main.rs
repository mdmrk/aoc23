mod days;

use std::{env, error::Error, fmt::Display, num::ParseIntError};

use days::get_problems;

#[derive(Debug)]
struct AocError(String);

impl Display for AocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ParseIntError> for AocError {
    fn from(err: ParseIntError) -> Self {
        AocError(err.to_string())
    }
}

impl From<Box<dyn Error>> for AocError {
    fn from(err: Box<dyn Error>) -> Self {
        AocError(err.to_string())
    }
}

impl Error for AocError {}

#[derive(Debug)]
struct AocArgs {
    show_problem: bool,
    help: bool,
    day: Option<usize>,
}

fn parse_args() -> Result<AocArgs, AocError> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut aoc_args = AocArgs {
        show_problem: false,
        help: false,
        day: None,
    };

    if args.is_empty() {
        return Err(AocError("provide a day".to_string()));
    }
    for arg in args.iter() {
        if arg == "-p" || arg == "--program" {
            aoc_args.show_problem = true;
        } else if arg == "-h" || arg == "--help" {
            aoc_args.help = true;
        } else {
            let day: usize = arg.parse()?;
            if day == 0 {
                return Err(AocError("day must be inside [1, 25]".to_string()));
            }
            aoc_args.day = Some(day);
        }
    }
    Ok(aoc_args)
}

fn main() -> Result<(), AocError> {
    let help_msg = "aoc23 - Advent of Code 2023

Written in Rust

Usage: aoc23 <day> [options]
  
      Options:
          --problem, -p   Print day's problem to solve
          --version, -v   Print version string
          --help, -h      Print this message
    ";
    let aoc_args = parse_args()?;

    if aoc_args.help {
        println!("{}", help_msg);
        return Ok(());
    }

    if aoc_args.day.is_none() {
        return Err(AocError("please provide a day".to_string()));
    }

    let problems = get_problems();

    let day = aoc_args.day.unwrap();
    let i = day - 1;

    if day > problems.len() {
        return Err(AocError("day not implemented".to_string()));
    }

    if aoc_args.show_problem {
        println!("{}", problems[i].problem());
        return Ok(());
    }

    let part1_result = problems[i].part1().map_err(|e| AocError(e.to_string()))?;
    let part2_result = problems[i].part2().map_err(|e| AocError(e.to_string()))?;

    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);
    Ok(())
}
