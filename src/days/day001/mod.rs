use crate::days::Day;

pub struct Day001;

impl Day for Day001 {
    fn problem(&self) -> &'static str {
        include_str!("problem.txt")
    }
    fn input(&self) -> &'static str {
        include_str!("input.txt")
    }
    fn part1(&self, input: &String) -> Result<String, String> {
        let mut result: usize = 0;

        for line in input.lines() {
            let first_idx = line.to_string().find(|c: char| c.is_numeric());
            let last_idx = line.to_string().rfind(|c: char| c.is_numeric());
            let first_num = line.chars().nth(first_idx.unwrap()).unwrap();
            let last_num = line.chars().nth(last_idx.unwrap()).unwrap();
            let mut combined = "".to_string();
            combined.push(first_num);
            combined.push(last_num);
            let num: usize = combined.parse().unwrap();
            result += num;
        }
        Ok(result.to_string())
    }

    fn part2(&self, input: &String) -> Result<String, String> {
        let cloned_in = input.to_string();
        //         let cloned_in = "two1nine
        // eightwothree
        // abcone2threexyz
        // xtwone3four
        // 4nineeightseven2
        // zoneight234
        // 7pqrstsixteen"
        //             .to_string();
        let mut proc_lines = Vec::new();
        let mut result: usize = 0;

        for line in cloned_in.lines() {
            let mut current_line = line.to_string();
            loop {
                let finds: [(usize, Option<usize>); 9] = [
                    (1, current_line.find("one")),
                    (2, current_line.find("two")),
                    (3, current_line.find("three")),
                    (4, current_line.find("four")),
                    (5, current_line.find("five")),
                    (6, current_line.find("six")),
                    (7, current_line.find("seven")),
                    (8, current_line.find("eight")),
                    (9, current_line.find("nine")),
                ];
                if finds.iter().all(|(_, f)| f.is_none()) {
                    break;
                } else {
                    let (i, _) = finds
                        .iter()
                        .filter(|(_, f)| f.is_some())
                        .min_by_key(|(_, f)| f.unwrap())
                        .unwrap();
                    match i {
                        1 => current_line = current_line.replacen("one", "1", 1),
                        2 => current_line = current_line.replacen("two", "2", 1),
                        3 => current_line = current_line.replacen("three", "3", 1),
                        4 => current_line = current_line.replacen("four", "4", 1),
                        5 => current_line = current_line.replacen("five", "5", 1),
                        6 => current_line = current_line.replacen("six", "6", 1),
                        7 => current_line = current_line.replacen("seven", "7", 1),
                        8 => current_line = current_line.replacen("eight", "8", 1),
                        9 => current_line = current_line.replacen("nine", "9", 1),
                        _ => unreachable!(),
                    }
                }
            }
            proc_lines.push(current_line);
        }

        for (_, line) in proc_lines.iter().enumerate() {
            let first_idx = line.to_string().find(|c: char| c.is_numeric());
            let last_idx = line.to_string().rfind(|c: char| c.is_numeric());
            let first_num = line.chars().nth(first_idx.unwrap()).unwrap();
            let last_num = line.chars().nth(last_idx.unwrap()).unwrap();
            let mut combined = "".to_string();
            combined.push(first_num);
            combined.push(last_num);
            let num: usize = combined.parse().unwrap();
            result += num;
        }
        Ok(result.to_string())
    }
}
