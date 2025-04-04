pub mod day001;

pub fn get_problems() -> Vec<Box<dyn Day>> {
    vec![Box::new(day001::Day001)]
}

pub trait Day {
    fn problem(&self) -> &'static str;
    fn input(&self) -> &'static str;
    fn part1(&self, _: &String) -> Result<String, String> {
        Ok("".to_string())
    }
    fn part2(&self, _: &String) -> Result<String, String> {
        Ok("".to_string())
    }
}
