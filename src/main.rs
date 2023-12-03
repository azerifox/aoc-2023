use std::env;

use aoc_solutions::{execute_day, Part};

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() || args.iter().count() != 2 {
        return Err(create_help_text());
    }

    let day: u8 = args[0]
        .parse()
        .map_err(|_| String::from("First argument should have been a number"))?;

    let part: u8 = args[1]
        .parse()
        .map_err(|_| String::from("Second argument should have been a number"))?;

    let part = match part {
        1 => Ok(Part::One),
        2 => Ok(Part::Two),
        _ => Err(String::from("Only 1 or 2 allowed for Part parameter")),
    }?;

    let result = execute_day(day, part)?;
    println!("answer: {}", result);

    Ok(())
}

fn create_help_text() -> String {
    String::from(
        "Invalid arguments, expected usage:
        aoc-2023 [day] [part]",
    )
}
