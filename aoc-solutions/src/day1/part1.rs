pub fn solve(input: &str) -> Result<String, String> {
    let lines: Vec<&str> = input.split("\n").collect();

    let mut calibration_values: Vec<u32> = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        };

        let first_digit = find_first_digit(line)?;
        let last_digit = find_last_digit(line)?;

        let line_value = first_digit.to_string() + &last_digit.to_string();
        let line_value: u32 = line_value
            .parse()
            .map_err(|_| "the line value should be parsable into a number".to_string())?;

        calibration_values.push(line_value);
    }

    let sum: u32 = calibration_values.iter().sum();
    Ok(sum.to_string())
}

fn find_first_digit(input_line: &str) -> Result<char, String> {
    for character in input_line.chars() {
        match character.is_digit(10) {
            true => return Ok(character),
            false => continue,
        };
    }

    Err(String::from(
        "Didn't find any digit, there should have been at least one",
    ))
}

fn find_last_digit(input_line: &str) -> Result<char, String> {
    for character in input_line.chars().rev() {
        match character.is_digit(10) {
            true => return Ok(character),
            false => continue,
        };
    }

    Err(String::from(
        "Didn't find any digit, there should have been at least one",
    ))
}
