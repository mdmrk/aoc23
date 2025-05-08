pub mod day001;
pub mod day002;
pub mod day003;
pub mod day004;
pub mod day005;
pub mod day006;
pub mod day007;
pub mod day008;
pub mod day009;
pub mod day010;
pub mod day011;
pub mod day012;
pub mod day013;
pub mod day014;
pub mod day015;
pub mod day016;
pub mod day017;
pub mod day018;
pub mod day019;
pub mod day020;
pub mod day021;
pub mod day022;
pub mod day023;
pub mod day024;
pub mod day025;

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
        Box::new(day010::Day010),
        Box::new(day011::Day011),
        Box::new(day012::Day012),
        Box::new(day013::Day013),
        Box::new(day014::Day014),
        Box::new(day015::Day015),
        Box::new(day016::Day016),
        Box::new(day017::Day017),
        Box::new(day018::Day018),
        Box::new(day019::Day019),
        Box::new(day020::Day020),
        Box::new(day021::Day021),
        Box::new(day022::Day022),
        Box::new(day023::Day023),
        Box::new(day024::Day024),
        Box::new(day025::Day025),
    ]
}

pub trait Day {
    fn problem(&self) -> &'static str;
    fn input(&self) -> &'static str;
    fn part1(&self, _: &String) -> Result<String, String> {
        Err("not implemented".to_string())
    }
    fn part2(&self, _: &String) -> Result<String, String> {
        Err("not implemented".to_string())
    }
}
