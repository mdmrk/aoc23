use std::collections::HashMap;

use crate::days::Day;

pub struct Day005;

impl Day for Day005 {
    fn problem(&self) -> &'static str {
        include_str!("problem.txt")
    }
    fn input(&self) -> &'static str {
        include_str!("input.txt")
    }
    fn part1(&self, input: &String) -> Result<String, String> {
        let mut result: usize = 0;

        struct Entry {
            destination_range_start: usize,
            source_range_start: usize,
            range_len: usize,
        }

        let mut lines = input.lines();

        let seeds = lines
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .map(|n| (n.parse::<usize>().unwrap(), None))
            .collect::<Vec<(usize, Option<usize>)>>();

        assert!(lines.next().unwrap().is_empty());

        let mut map: HashMap<usize, Option<usize>> = seeds.clone().into_iter().collect();
        println!("seeds {:?}\n\n", seeds);
        loop {
            match lines.next() {
                Some(line) => {
                    if line.is_empty() {
                        let fmap = map.iter().map(|(k, v)| (v.unwrap_or(*k), None)).collect();
                        println!("fffff mapped {:?}\n", fmap);
                        map = fmap;
                    } else {
                        if line.chars().next().unwrap().is_digit(10) {
                            println!("mapped {:?}", map);
                            let nums = line
                                .split_whitespace()
                                .map(|n| n.parse::<usize>().unwrap())
                                .collect::<Vec<usize>>();
                            assert!(nums.len() == 3);

                            let entry = Entry {
                                destination_range_start: nums[0],
                                source_range_start: nums[1],
                                range_len: nums[2],
                            };

                            map = map
                                .iter()
                                .map(|(k, v)| {
                                    if v.is_some() {
                                        (*k, *v)
                                    } else {
                                        if *k >= entry.source_range_start
                                            && *k < entry.source_range_start + entry.range_len
                                        {
                                            let i = *k - entry.source_range_start;
                                            (*k, Some(entry.destination_range_start + i))
                                        } else {
                                            (*k, None)
                                        }
                                    }
                                })
                                .collect();
                        }
                    }
                }
                None => break,
            }
        }
        let min = map.iter().map(|(k, _)| k).min().unwrap().to_owned();
        println!("mapped {:?}", map);

        result = min;
        Ok(result.to_string())
    }

    fn part2(&self, input: &String) -> Result<String, String> {
        let result: usize = 0;

        Ok(result.to_string())
    }
}
