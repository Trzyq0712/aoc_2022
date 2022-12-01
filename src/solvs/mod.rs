pub use super::utils::Scanner;
use std::fs::read_to_string;

mod day1;
mod day2;

pub trait Solution {
    fn file_name(&self) -> &'static str;
    fn solve(&self, input: &str);
    fn run(&self) -> Result<(), &'static str> {
        let input = read_to_string(self.file_name()).map_err(|_| "Could not read from file")?;
        self.solve(&input);
        Ok(())
    }
}

pub fn get_solution(day: u32) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Box::new(day1::Day1)),
        2 => Some(Box::new(day2::Day2)),
        _ => None,
    }
}
