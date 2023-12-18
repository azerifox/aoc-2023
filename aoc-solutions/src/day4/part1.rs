use std::collections::HashSet;

pub fn solve(input: &str) -> Result<String, String> {
    let mut sum = 0;
    // string slices actually have a lines() method available, nice!
    for line in input.lines() {
        let (winning_numbers, scratched_numbers) = get_card_numbers(line);

        let winning_numbers: HashSet<u8> = HashSet::from_iter(winning_numbers);

        let scratched_winners = scratched_numbers
            .iter()
            .filter(|&scratched_number| winning_numbers.contains(scratched_number))
            .count() as u32;

        if scratched_winners > 0 {
            sum += usize::pow(2, scratched_winners - 1);
        }
    }

    Ok(sum.to_string())
}

pub(super) fn get_card_numbers(line: &str) -> (Vec<u8>, Vec<u8>) {
    let (_, numbers) = line.split_once(':').unwrap();
    let (winning_numbers, scratched_numbers) = numbers.split_once('|').unwrap();
    (
        winning_numbers
            .split_whitespace()
            .map(|ticket_number| ticket_number.parse().unwrap())
            .collect::<Vec<u8>>(),
        scratched_numbers
            .split_whitespace()
            .map(|ticket_number| ticket_number.parse().unwrap())
            .collect::<Vec<u8>>(),
    )
}
