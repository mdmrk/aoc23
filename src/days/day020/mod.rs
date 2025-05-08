use crate::days::Day;
pub struct Day020;
impl Day for Day020 {
    fn problem(&self) -> &'static str {
        include_str!("problem.txt")
    }
    fn input(&self) -> &'static str {
        include_str!("input.txt")
    }
    fn part1(&self, input: &String) -> Result<String, String> {
        let mut result: usize = 0;
        Ok(result.to_string())
    }
    fn part2(&self, input: &String) -> Result<String, String> {
        let result: usize = 0;
        Ok(result.to_string())
    }
}
