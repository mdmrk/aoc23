pub mod day001;

pub fn get_problems() -> Vec<Box<dyn Day>> {
    vec![Box::new(day001::Day001)]
}

pub trait Day {
    fn problem(&self) -> &'static str;
    fn part1(&self) -> Result<String, String> {
        unimplemented!()
    }
    fn part2(&self) -> Result<String, String> {
        unimplemented!()
    }
}
