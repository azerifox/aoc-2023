use day1::Day1;
use day2::Day2;
use std::fs;

pub mod day1;
pub mod day2;

pub trait ExecutePart {
    fn execute_part(&self, input: &str, part: Part) -> Result<String, String>;
}

pub enum Part {
    One,
    Two,
}

pub struct Day<T: ExecutePart> {
    input_file: String,
    solution: T,
}

impl<T: ExecutePart> Day<T> {
    pub fn new(input_file: String, solution: T) -> Day<T> {
        Day {
            input_file,
            solution,
        }
    }

    pub fn execute(&self, part: Part) -> Result<String, String> {
        let input = fs::read_to_string(&self.input_file).map_err(|err| err.to_string())?;

        self.solution.execute_part(&input, part)
    }
}

pub fn execute_day(day: u8, part: Part) -> Result<String, String> {
    match day {
        1 => Day::new(String::from("aoc-solutions/input/01"), Day1).execute(part),
        2 => Day::new(String::from("aoc-solutions/input/02"), Day2).execute(part),
        _ => Err("Day not implemented".to_string()),
    }
}
