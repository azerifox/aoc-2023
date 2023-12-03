pub mod part1;
pub mod part2;

pub struct Day1;

impl crate::ExecutePart for Day1 {
    fn execute_part(&self, input: &str, part: crate::Part) -> Result<String, String> {
        match part {
            crate::Part::One => self::part1::solve(input),
            crate::Part::Two => self::part2::solve(input),
        }
    }
}
