use crate::days::Day;
use std::ptr::null;
pub struct Day015;
impl Day for Day015 {
    fn problem(&self) -> &'static str {
        include_str!("problem.txt")
    }
    fn input(&self) -> &'static str {
        include_str!("input.txt")
    }
    fn part1(&self, input: &String) -> Result<String, String> {
        let mut result: usize = 0;
        let mut total: usize = 0;
        for w in input.trim().split(',') {
            let mut value: usize = 0;
            for c in w.chars() {
                let ascii = c as u8 as usize;
                value += ascii;
                value *= 17;
                value %= 256;
            }
            total += value;
        }
        result = total;
        Ok(result.to_string())
    }
    fn part2(&self, input: &String) -> Result<String, String> {
        let mut result: usize = 0;
        type Lens = (String, usize);
        type Boxx = (usize, Vec<Lens>);
        let mut boxes: [Boxx; 256] = core::array::from_fn(|_| (0, vec![]));
        {
            for (i, b) in boxes.iter_mut().enumerate() {
                b.0 = i;
            }
        }
        for w in input.trim().split(',') {
            let mut value: usize = 0;
            let is_equal: bool = w.find("=").is_some();
            let tag = if is_equal {
                w.split_once('=').unwrap().0
            } else {
                w.split_once('-').unwrap().0
            };
            let n: usize = if is_equal {
                w.split_once('=').unwrap().1.parse().unwrap()
            } else {
                0
            };
            for c in tag.chars() {
                let ascii = c as u8 as usize;
                value += ascii;
                value *= 17;
                value %= 256;
            }
            let box_i = value;
            if !is_equal {
                boxes[box_i].1.retain(|lens| lens.0 != tag);
            } else {
                let mut thereis: bool = false;
                for bc in boxes[box_i].1.iter_mut() {
                    if bc.0 == tag {
                        bc.1 = n;
                        thereis = true;
                    }
                }
                if !thereis {
                    boxes[box_i].1.push((tag.to_string(), n));
                }
            }
            // println!(
            //     "{:?}",
            //     boxes
            //         .clone()
            //         .into_iter()
            //         .filter(|b| !b.1.is_empty())
            //         .collect::<Vec<Boxx>>()
            // );
        }

        for b in boxes {
            let a = b.0 + 1;
            for (j, l) in b.1.iter().enumerate() {
                let bb = j + 1;
                let r = a * bb * l.1;
                result += r;
            }
        }

        Ok(result.to_string())
    }
}
