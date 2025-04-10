
use crate::days::Day;

pub struct Day003;

impl Day for Day003 {
    fn problem(&self) -> &'static str {
        include_str!("problem.txt")
    }
    fn input(&self) -> &'static str {
        include_str!("input.txt")
    }
    fn part1(&self, input: &String) -> Result<String, String> {
        let mut result: usize = 0;

        let rows = input.lines().collect::<Vec<&str>>();

        for (i, row) in rows.iter().enumerate() {
            let mut find_str = row.to_string();

            let nums = row
                .split(|c: char| !c.is_numeric())
                .filter(|n| !n.is_empty())
                .collect::<Vec<&str>>();

            for num in &nums {
                let j = find_str.find(num).unwrap();
                let mut ok = false;
                let next_i = j + num.chars().collect::<Vec<char>>().len();

                for (o, _) in num.chars().enumerate() {
                    let n = o + j;

                    let mut i1: isize = -1;
                    while i1 <= 1 {
                        let mut j1: isize = -1;

                        while j1 <= 1 {
                            let i2 = i as isize + i1;
                            let j2 = n as isize + j1;

                            if i2 >= 0
                                && j2 >= 0
                                && i2 < rows.len() as isize
                                && j2 < row.len() as isize
                            {
                                let i3 = usize::try_from(i2).unwrap();
                                let j3 = usize::try_from(j2).unwrap();
                                // println!("digit: {} looking i: {} j: {}", o, i3, j3);
                                let c = rows.iter().nth(i3).unwrap().chars().nth(j3).unwrap();
                                if !c.is_numeric() && c != '.' {
                                    ok = true;
                                }
                            }
                            j1 += 1;
                        }
                        i1 += 1;
                    }
                }
                if ok {
                    result += num.parse::<usize>().unwrap();
                }
                // println!(
                //     "num: {} i: {} next_i: {} ok: {}\nfind_str: {}",
                //     num, j, next_i, ok, find_str
                // );
                find_str = find_str.replacen(num, &".".repeat(num.len()), 1);
            }
        }

        Ok(result.to_string())
    }

    fn part2(&self, input: &String) -> Result<String, String> {
        let result: usize = 0;

        let rows = input.chars().map(|c| if c.is_numeric() { c } else { '\0' });
        println!("{:?}", rows);

        Ok(result.to_string())
    }
}
