use crate::day3::part1;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Result<String, String> {
    let mut numbers: Vec<part1::Number> = Vec::new();
    let mut symbols: Vec<part1::Symbol> = Vec::new();
    let mut row: u8 = 0;

    for line in input.split("\n").filter(|&line| !line.is_empty()) {
        part1::extract_content(line, row, &mut numbers, &mut symbols);
        row += 1;
    }

    Ok(sum_part_numbers(&numbers, &symbols).to_string())
}

fn sum_part_numbers(numbers: &Vec<part1::Number>, symbols: &Vec<part1::Symbol>) -> u32 {
    let mut symbol_positions = HashSet::new();
    let mut number_positions = HashMap::new();

    for symbol in symbols.iter().filter(|&s| s.value == '*') {
        symbol_positions.insert((symbol.row, symbol.position));
    }

    for number in numbers {
        for position in &number.positions {
            number_positions.insert((number.row, *position), number);
        }
    }

    let mut sum: u32 = 0;

    for (symbol_row, symbol_position) in symbol_positions {
        let mut adjacent_numbers: Vec<u16> = Vec::new();

        // left
        if let Some(&number) = number_positions.get(&(symbol_row, symbol_position - 1)) {
            adjacent_numbers.push(number.value);
        };

        // right
        if let Some(&number) = number_positions.get(&(symbol_row, symbol_position + 1)) {
            adjacent_numbers.push(number.value);
        };

        let mut top: Vec<Option<&&part1::Number>> = Vec::new();
        let mut bottom: Vec<Option<&&part1::Number>> = Vec::new();

        for position in symbol_position - 1..=symbol_position + 1 {
            top.push(number_positions.get(&(symbol_row - 1, position)));
            bottom.push(number_positions.get(&(symbol_row + 1, position)));
        }

        let mut unique_numbers = HashSet::new();

        for maybe_number in top.iter().chain(bottom.iter()) {
            if let Some(&number) = maybe_number {
                let pointer = number as *const part1::Number;
                if unique_numbers.insert(pointer) {
                    adjacent_numbers.push(number.value);
                }
            }
        }

        if adjacent_numbers.iter().count() == 2 {
            sum += adjacent_numbers
                .iter()
                .fold(1, |product, &number| product * number as u32);
            // access by index might not be as safe and sound as fold but probably way more readable:
            // adjacent_numbers[0] as u32 * adjacent_number[1] as u32 for comparison
            // Alternatives?
        }
    }

    sum
}
