use crate::days::Day;

pub struct Day009;

fn helper(v: &Vec<isize>) -> isize {
    // println!("{:?}", v);
    if v.iter().all(|f| *f == 0) {
        0
    } else {
        let mut v2: Vec<isize> = vec![];
        let mut i: usize = 0;
        while i < v.len() - 1 {
            v2.push(v[i + 1] - v[i]);
            i += 1;
        }
        let diff = v[i];
        // println!("{}", diff);
        diff + helper(&v2)
    }
}

fn helper2(v: &Vec<isize>) -> isize {
    // println!("{:?}", v);
    if v.iter().all(|f| *f == 0) {
        0
    } else {
        let mut v2: Vec<isize> = vec![];
        let mut i: usize = 0;
        while i < v.len() - 1 {
            v2.push(v[i + 1] - v[i]);
            i += 1;
        }
        let diff = v[0];
        // println!("{}", diff);
        diff - helper2(&v2)
    }
}

impl Day for Day009 {
    fn problem(&self) -> &'static str {
        include_str!("problem.txt")
    }
    fn input(&self) -> &'static str {
        include_str!("input.txt")
    }
    fn part1(&self, input: &String) -> Result<String, String> {
        let mut result: usize = 0;

        let entries: Vec<Vec<isize>> = input
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .map(|n| n.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>()
            })
            .collect();

        result = entries.iter().fold(0 as isize, |a, e| a + helper(&e)) as usize;
        Ok(result.to_string())
    }

    fn part2(&self, input: &String) -> Result<String, String> {
        let mut result: usize = 0;

        let mut entries: Vec<Vec<isize>> = input
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .map(|n| n.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>()
            })
            .collect();

        result = entries.iter_mut().fold(0 as isize, |a, e| a + helper2(&e)) as usize;
        Ok(result.to_string())
    }
}
