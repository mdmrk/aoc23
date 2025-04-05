use crate::days::Day;

pub struct Day002;

impl Day for Day002 {
    fn problem(&self) -> &'static str {
        include_str!("problem.txt")
    }
    fn input(&self) -> &'static str {
        include_str!("input.txt")
    }
    fn part1(&self, input: &String) -> Result<String, String> {
        let mut result: usize = 0;

        for line in input.lines() {
            let mut valid = true;
            let (game, right) = line.split_once(":").unwrap();
            let game_id: usize = game.split_once(" ").unwrap().1.parse().unwrap();
            right.split(";").for_each(|cube_set| {
                cube_set.split(",").for_each(|cubes| {
                    let (count, color) = cubes.strip_prefix(" ").unwrap().split_once(" ").unwrap();
                    let c: usize = count.parse().unwrap();
                    let maximum: usize = match color {
                        "blue" => 14,
                        "red" => 12,
                        "green" => 13,
                        _ => unreachable!(),
                    };
                    if c > maximum {
                        valid = false;
                    }
                })
            });
            if valid {
                result += game_id;
            }
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &String) -> Result<String, String> {
        let mut result: usize = 0;

        for line in input.lines() {
            let (_, right) = line.split_once(":").unwrap();
            // let game_id: usize = game.split_once(" ").unwrap().1.parse().unwrap();
            let mut blues: usize = std::usize::MIN;
            let mut greens: usize = std::usize::MIN;
            let mut reds: usize = std::usize::MIN;
            right.split(";").for_each(|cube_set| {
                let cubes: Vec<&str> = cube_set.split(",").collect();

                cubes.iter().for_each(|cubes| {
                    let (count, color) = cubes.strip_prefix(" ").unwrap().split_once(" ").unwrap();
                    let c: usize = count.parse().unwrap();
                    match color {
                        "blue" => {
                            if c > blues {
                                blues = c;
                            }
                        }
                        "red" => {
                            if c > reds {
                                reds = c;
                            }
                        }
                        "green" => {
                            if c > greens {
                                greens = c;
                            }
                        }
                        _ => unreachable!(),
                    };
                });
            });
            let s = blues * greens * reds;
            result += s;
        }

        Ok(result.to_string())
    }
}
