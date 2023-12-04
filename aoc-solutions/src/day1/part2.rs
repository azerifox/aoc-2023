pub fn solve(input: &str) -> Result<String, String> {
    let lines: Vec<&str> = input
        .split("\n")
        .take_while(|&line| !line.is_empty())
        .collect();

    let mut calibration_values: Vec<u32> = Vec::new();

    for line in lines {
        let first_digit = find_digit(&line, SearchDirection::Right);
        let last_digit = find_digit(&line, SearchDirection::Left);
        let first_spelled_digit = find_spelled_digit(&line, SearchDirection::Right);
        let last_spelled_digit = find_spelled_digit(&line, SearchDirection::Left);

        let tens = calibration_value_tens(&first_digit, &first_spelled_digit);
        let ones = calibration_value_ones(&last_digit, &last_spelled_digit);
        let calibration_value = tens + ones;

        calibration_values.push(calibration_value as u32);

        print!("{} <> ", &calibration_value);
        println!("{}", &line);
    }

    let sum: u32 = calibration_values.iter().sum();
    Ok(sum.to_string())
}

struct SpelledOutDigit {
    spelling: &'static str,
    digit: u8,
}

#[derive(Debug)]
struct DigitSearchResult {
    byte_index: usize,
    digit: u8,
}

enum SearchDirection {
    Right,
    Left,
}

const SPELLED_DIGITS: &'static [SpelledOutDigit] = &[
    SpelledOutDigit {
        spelling: "one",
        digit: 1,
    },
    SpelledOutDigit {
        spelling: "two",
        digit: 2,
    },
    SpelledOutDigit {
        spelling: "three",
        digit: 3,
    },
    SpelledOutDigit {
        spelling: "four",
        digit: 4,
    },
    SpelledOutDigit {
        spelling: "five",
        digit: 5,
    },
    SpelledOutDigit {
        spelling: "six",
        digit: 6,
    },
    SpelledOutDigit {
        spelling: "seven",
        digit: 7,
    },
    SpelledOutDigit {
        spelling: "eight",
        digit: 8,
    },
    SpelledOutDigit {
        spelling: "nine",
        digit: 9,
    },
];

fn find_digit(input_line: &str, direction: SearchDirection) -> Option<DigitSearchResult> {
    // Variable is trait typed but the different trait implementations require dynamic dispatch so it's boxed.
    // Heap allocation and dynamic dispatch is totally fine in this case but certainly unexpected to be required
    // when combining the two methods from part 1
    // (for loop would otherwise use static dispatch with the iterator type known at compile time)
    // Interesting delve into rust, I leave it like this as learning history in case I revisit the solution
    let char_iter = init_search_iter(&input_line, direction);

    for (position, character) in char_iter {
        match character.is_digit(10) {
            true => {
                return Some(DigitSearchResult {
                    byte_index: position,
                    digit: character.to_string().parse().unwrap(),
                });
            }
            false => {
                continue;
            }
        };
    }

    None
}

fn find_spelled_digit(input_line: &str, direction: SearchDirection) -> Option<DigitSearchResult> {
    let mut spelled_digits: Vec<DigitSearchResult> = Vec::new();

    for spelled_digit in SPELLED_DIGITS {
        let occurrence = match direction {
            SearchDirection::Right => input_line.find(spelled_digit.spelling),
            SearchDirection::Left => input_line.rfind(spelled_digit.spelling),
        };
        match occurrence {
            Some(index) => {
                spelled_digits.push(DigitSearchResult {
                    byte_index: index,
                    digit: spelled_digit.digit,
                });
                continue;
            }
            None => continue,
        }
    }

    if spelled_digits.is_empty() {
        None
    } else {
        spelled_digits.sort_by(|left, right| left.byte_index.cmp(&right.byte_index));
        match direction {
            SearchDirection::Right => spelled_digits.into_iter().next(),
            SearchDirection::Left => spelled_digits.into_iter().rev().next(),
        }
    }
}

fn init_search_iter(
    input_line: &str,
    direction: SearchDirection,
) -> Box<dyn Iterator<Item = (usize, char)> + '_> {
    match direction {
        SearchDirection::Right => Box::new(input_line.char_indices()),
        SearchDirection::Left => Box::new(input_line.char_indices().rev()),
    }
}

fn calibration_value_tens(
    digit_result: &Option<DigitSearchResult>,
    spelled_digit_result: &Option<DigitSearchResult>,
) -> u8 {
    if let Some(digit_search) = digit_result {
        match spelled_digit_result {
            Some(spelled_search) => {
                if digit_search.byte_index < spelled_search.byte_index {
                    digit_search.digit * 10
                } else {
                    spelled_search.digit * 10
                }
            }
            None => digit_search.digit * 10,
        }
    } else {
        match spelled_digit_result {
            Some(spelled_search) => spelled_search.digit * 10,
            None => {
                panic!("both tens search results missing")
            }
        }
    }
}

fn calibration_value_ones(
    digit_result: &Option<DigitSearchResult>,
    spelled_digit_result: &Option<DigitSearchResult>,
) -> u8 {
    if let Some(digit_search) = digit_result {
        match spelled_digit_result {
            Some(spelled_search) => {
                if digit_search.byte_index > spelled_search.byte_index {
                    digit_search.digit
                } else {
                    spelled_search.digit
                }
            }
            None => digit_search.digit,
        }
    } else {
        match spelled_digit_result {
            Some(spelled_search) => spelled_search.digit,
            None => panic!("both tens search results missing"),
        }
    }
}
