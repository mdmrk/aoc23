mod days;

use std::{env, error::Error, fmt::Display, fs, path::Path};

use days::get_problems;

#[derive(Debug)]
struct AocError(String);

impl Display for AocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for AocError {}

#[derive(Debug)]
struct AocArgs {
    show_problem: bool,
    help: bool,
    day: Option<usize>,
    input: Option<String>,
}

fn parse_args() -> Result<AocArgs, AocError> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut aoc_args = AocArgs {
        show_problem: false,
        input: None,
        help: false,
        day: None,
    };

    if args.is_empty() {
        return Err(AocError("provide a day".to_string()));
    }
    let mut iter = args.iter();
    loop {
        match iter.next() {
            Some(arg) => {
                if arg.starts_with("-") {
                    if arg == "-p" || arg == "--program" {
                        aoc_args.show_problem = true;
                    } else if arg == "-h" || arg == "--help" {
                        aoc_args.help = true;
                    } else if arg == "-i" || arg == "--input" {
                        if aoc_args.input.is_some() {
                            return Err(AocError("multiple inputs are not supported".to_string()));
                        }
                        let filepath = iter
                            .next()
                            .ok_or(AocError("specify input filename".to_string()))?;
                        let path = Path::new(filepath);
                        let contents = fs::read_to_string(path).unwrap();
                        aoc_args.input = Some(contents);
                    } else {
                        return Err(AocError("invalid argument".to_string()));
                    }
                } else {
                    let day: usize = arg.parse().unwrap();
                    if day == 0 {
                        return Err(AocError("day must be inside [1, 25]".to_string()));
                    }
                    aoc_args.day = Some(day);
                }
            }
            None => break,
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
          --input, -i     Set input for a problem
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

    let problem = &problems[i];
    if aoc_args.show_problem {
        println!("{}", problem.problem());
        return Ok(());
    }

    let input: String = if aoc_args.input.is_some() {
        aoc_args.input.unwrap()
    } else {
        problem.input().to_string()
    };

    let part1_result = problem.part1(&input).map_err(|e| AocError(e.to_string()))?;
    let part2_result = problem.part2(&input).map_err(|e| AocError(e.to_string()))?;

    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);

    Ok(())
}
