use std::collections::HashMap;

use crate::days::Day;

pub struct Day006;

impl Day for Day006 {
    fn problem(&self) -> &'static str {
        include_str!("problem.txt")
    }
    fn input(&self) -> &'static str {
        include_str!("input.txt")
    }
    fn part1(&self, input: &String) -> Result<String, String> {
        let mut result: usize = 1;

        #[derive(Debug)]
        struct Race {
            time: usize,
            distance: usize,
        }
        let mut races: Vec<Race> = vec![];

        let (times, distances) = input
            .split_once("\n")
            .map(|(t, d)| {
                (
                    t.strip_prefix("Time: ")
                        .unwrap()
                        .split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>(),
                    d.strip_prefix("Distance: ")
                        .unwrap()
                        .split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>(),
                )
            })
            .unwrap();

        assert_eq!(times.len(), distances.len());

        for i in 0..times.len() {
            races.push(Race {
                time: times[i],
                distance: distances[i],
            });
        }
        // println!("{:?}", races);

        for race in races {
            let mut wins: usize = 0;

            for t in 1..race.time {
                let acc_time = t;
                let time_left = race.time - t;
                let distance = acc_time * time_left;

                if distance > race.distance {
                    // println!("{}", t);
                    wins += 1;
                }
            }
            // println!("{}\n", wins);
            result *= wins;
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &String) -> Result<String, String> {
        let mut result: usize = 1;

        #[derive(Debug)]
        struct Race {
            time: usize,
            distance: usize,
        }

        let race = input
            .split_once("\n")
            .map(|(t, d)| Race {
                time: t
                    .strip_prefix("Time: ")
                    .unwrap()
                    .replace(" ", "")
                    .trim()
                    .parse::<usize>()
                    .unwrap(),
                distance: d
                    .strip_prefix("Distance: ")
                    .unwrap()
                    .replace(" ", "")
                    .trim()
                    .parse::<usize>()
                    .unwrap(),
            })
            .unwrap();

        // println!("{:?}", race);

        let mut wins: usize = 0;

        for t in 1..race.time {
            let acc_time = t;
            let time_left = race.time - t;
            let distance = acc_time * time_left;

            if distance > race.distance {
                // println!("{}", t);
                wins += 1;
            }
        }
        // println!("{}\n", wins);
        result *= wins;

        Ok(result.to_string())
    }
}
