pub mod day001;
pub mod day002;
pub mod day003;
pub mod day004;
pub mod day005;
pub mod day006;
pub mod day007;
pub mod day008;
pub mod day009;

pub fn get_problems() -> Vec<Box<dyn Day>> {
    vec![
        Box::new(day001::Day001),
        Box::new(day002::Day002),
        Box::new(day003::Day003),
        Box::new(day004::Day004),
        Box::new(day005::Day005),
        Box::new(day006::Day006),
        Box::new(day007::Day007),
        Box::new(day008::Day008),
        Box::new(day009::Day009),
    ]
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
