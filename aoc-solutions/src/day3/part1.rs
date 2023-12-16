use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Result<String, String> {
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut row: u8 = 0;

    for line in input.split("\n").filter(|&line| !line.is_empty()) {
        extract_content(line, row, &mut numbers, &mut symbols);
        row += 1;
    }

    Ok(sum_part_numbers(&numbers, &symbols).to_string())
}

pub(super) struct Number {
    pub(super) value: u16,
    pub(super) positions: Vec<u8>,
    pub(super) row: u8,
}

pub(super) struct Symbol {
    pub(super) value: char,
    pub(super) position: u8,
    pub(super) row: u8,
}

pub(super) fn extract_content(
    line: &str,
    row: u8,
    numbers: &mut Vec<Number>,
    symbols: &mut Vec<Symbol>,
) {
    let mut position: u8 = 0;
    let mut tracking_number = false;
    let mut number_buffer: Vec<char> = Vec::new();
    let mut position_buffer: Vec<u8> = Vec::new();

    /*
      Learned more about rust closures after I was unhappy with my first iteration of pretty ugly,
      repetitive code inside the loop's branches. Branching logic improved in the process as well.
      With both changes applied, this is actually readable.
      Only that the closure needs to be declared as mutable is a little counter-intuitive to me
      compared to the usual use of mut. The closure itself doesn't change but it captures
      mutable references as arguments and that's enough to require a mutable declaration for the
      closure. Meh...
    */
    let mut push_number = |number_buffer: &mut Vec<char>, position_buffer: &mut Vec<u8>| {
        numbers.push(Number {
            value: number_buffer.iter().collect::<String>().parse().unwrap(),
            positions: position_buffer.clone(),
            row,
        });
        number_buffer.clear();
        position_buffer.clear();
    };

    for character in line.chars() {
        if character.is_digit(10) {
            tracking_number = true;
            number_buffer.push(character);
            position_buffer.push(position);
        } else {
            if tracking_number {
                push_number(&mut number_buffer, &mut position_buffer);
                tracking_number = false;
            }

            if character != '.' {
                symbols.push(Symbol {
                    value: character,
                    position: position,
                    row,
                });
            }
        }
        position += 1;
    }

    if tracking_number {
        push_number(&mut number_buffer, &mut position_buffer);
    }
}

fn sum_part_numbers(numbers: &Vec<Number>, symbols: &Vec<Symbol>) -> u32 {
    let mut symbol_positions = HashSet::new();
    let mut number_positions = HashMap::new();

    for symbol in symbols {
        symbol_positions.insert((symbol.row, symbol.position));
    }

    for number in numbers {
        for position in &number.positions {
            number_positions.insert((number.row, *position), number);
        }
    }

    let mut sum: u32 = 0;

    for (symbol_row, symbol_position) in symbol_positions {
        let left = number_positions.get(&(symbol_row, symbol_position - 1));
        let right = number_positions.get(&(symbol_row, symbol_position + 1));
        let mut top: Vec<Option<&&Number>> = Vec::new();
        let mut bottom: Vec<Option<&&Number>> = Vec::new();

        for position in symbol_position - 1..=symbol_position + 1 {
            top.push(number_positions.get(&(symbol_row - 1, position)));
            bottom.push(number_positions.get(&(symbol_row + 1, position)));
        }

        sum += match left {
            Some(number) => number.value as u32,
            None => 0,
        };

        sum += match right {
            Some(number) => number.value as u32,
            None => 0,
        };

        let mut unique_numbers = HashSet::new();

        for maybe_number in top.iter().chain(bottom.iter()) {
            if let Some(&number) = maybe_number {
                let pointer = number as *const Number;
                if unique_numbers.insert(pointer) {
                    sum += number.value as u32;
                }
            }
        }
    }

    sum
}
