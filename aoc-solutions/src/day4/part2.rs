use crate::day4::part1;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Result<String, String> {
    let mut cards: HashMap<usize, usize> =
        (1..=input.lines().count()).map(|key| (key, 1)).collect();

    for (index, line) in input.lines().enumerate() {
        let current_card_number = index + 1;
        let (winning_numbers, scratched_numbers) = part1::get_card_numbers(line);

        let winning_numbers: HashSet<u8> = HashSet::from_iter(winning_numbers);

        let scratched_winners = scratched_numbers
            .iter()
            .filter(|&scratched_number| winning_numbers.contains(scratched_number))
            .count();

        if scratched_winners > 0 {
            let current_card_copies = *cards.entry(current_card_number).or_default();
            for card_number in current_card_number + 1..=current_card_number + scratched_winners {
                cards
                    .entry(card_number)
                    .and_modify(|card_count| *card_count += current_card_copies);
            }
        }
    }

    let total_cards: usize = cards.values().sum();

    Ok(total_cards.to_string())
}
