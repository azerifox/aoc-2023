pub fn solve(input: &str) -> Result<String, String> {
    let verification_number: u32 = input
        .split(',')
        .map(|step| calculate_hash(step.trim()))
        .sum();
    Ok(verification_number.to_string())
}

pub(super) fn calculate_hash(value: &str) -> u32 {
    value.chars().fold(0, |mut current_value, character| {
        current_value += character as u32;
        current_value *= 17;
        current_value %= 256;
        current_value
    })
}
