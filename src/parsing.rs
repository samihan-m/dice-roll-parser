use rand::Rng;
use regex::Regex;

/// Parse the dice roll string with a regex I tested via regex101.com
pub fn parse_roll_with_regex(input: &str) -> i32 {
    let re =
        Regex::new(r"(?<op>\+|-){0,1}((?<multiplier>\d*)d(?<die>\d+)|(?<scalar>\d+))").unwrap();
    let trimmed_string = input.replacen(' ', "", input.len());

    let mut rng = rand::thread_rng();

    let mut total = 0;

    for capture in re.captures_iter(trimmed_string.as_str()) {
        let op = capture.name("op");
        let multiplier = capture.name("multiplier");
        let die = capture.name("die");
        let scalar = capture.name("scalar");

        let mut value: i32 = 0;
        if let Some(die) = die {
            let die_size = die.as_str().parse::<i32>().unwrap();
            value = rng.gen_range(1..=die_size);

            if let Some(multiplier) = multiplier {
                let multiplier_string = multiplier.as_str();
                if !multiplier_string.is_empty() {
                    let multiplier = multiplier.as_str().parse::<i32>().unwrap();
                    value *= multiplier;
                }
            }
        }

        if let Some(scalar) = scalar {
            value += scalar.as_str().parse::<i32>().unwrap();
        }

        if let Some(op) = op {
            if op.as_str() == "-" {
                value = -value;
            }
        }

        total += value;
    }

    total
}

/// Errors that may arise during the parsing of a dice roll string
#[derive(Debug)]
pub enum Error {
    ParseError(String), // Unexpected character or something
    DieSideCountError(String), // Syntactically valid but semantically invalid die side count (e.g. a 0-sided die)
}

#[derive(Debug)]
enum State {
    NewTerm,
    ParsingNumber(String), // The string contains already-processed characters
    ParsingDieSideCount(i32, String), // The i32 is the number of this die to roll, and the string contains already-processed characters
}

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
}

/// Parse the dice roll string with something like a state machine I modelled off of a DFA I drew
pub fn parse_roll_with_state_machine(input: &str) -> Result<i32, Error> {
    let mut rng = rand::thread_rng();

    let mut state = State::NewTerm;
    let mut operation = Operation::Add;

    let mut total = 0;

    let character_iterator = input
        .chars()
        .filter(|char| !char.is_whitespace())
        .chain(std::iter::once(' ')); // Add a whitespace character to the end of the input to indicate end of input

    for (index, char) in character_iterator.enumerate() {
        // println!("State: {:?}, Char: {}, Total: {}", state, char, total);

        match state {
            State::NewTerm => match char {
                '0'..='9' => {
                    state = State::ParsingNumber(char.to_string());
                }
                'd' => {
                    state =
                        State::ParsingDieSideCount(1, String::with_capacity(input.len() - index));
                }
                '+' => operation = Operation::Add,
                '-' => operation = Operation::Subtract,
                ' ' => {}
                _ => {
                    return Err(Error::ParseError(format!(
                        "Unexpected character when looking for a new term: {}",
                        char
                    )))
                }
            },
            State::ParsingNumber(ref mut buffer) => match char {
                '0'..='9' => {
                    buffer.push(char);
                }
                'd' => {
                    let die_count = buffer.parse::<i32>();
                    match die_count {
                        Ok(die_count) => {
                            state = State::ParsingDieSideCount(
                                die_count,
                                String::with_capacity(input.len() - index),
                            );
                        }
                        Err(_) => {
                            return Err(Error::ParseError(format!(
                                "Failed to parse multiplier: {}",
                                buffer
                            )));
                        }
                    }
                }
                '+' | '-' | ' ' => {
                    let number = buffer.parse::<i32>();
                    match number {
                        Ok(number) => {
                            match operation {
                                Operation::Add => total += number,
                                Operation::Subtract => total -= number,
                            }

                            state = State::NewTerm;

                            match char {
                                '+' => operation = Operation::Add,
                                '-' => operation = Operation::Subtract,
                                ' ' => {}
                                _ => {
                                    return Err(Error::ParseError(format!(
                                        "Unexpected character when parsing number: {}",
                                        char
                                    )));
                                }
                            }
                        }
                        Err(_) => {
                            return Err(Error::ParseError(format!(
                                "Failed to parse number: {}",
                                buffer
                            )));
                        }
                    }
                }
                _ => {
                    return Err(Error::ParseError(format!(
                        "Unexpected character when parsing number: {}",
                        char
                    )))
                }
            },
            State::ParsingDieSideCount(die_roll_count, ref mut buffer) => match char {
                '0'..='9' => {
                    buffer.push(char);
                }
                '+' | '-' | ' ' => {
                    let die_side_count = buffer.parse::<i32>();
                    match die_side_count {
                        Ok(die_side_count) => {
                            if die_side_count < 1 {
                                return Err(Error::DieSideCountError(format!(
                                    "Die side-count should be at least 1. Instead, it was {}",
                                    die_side_count
                                )));
                            }

                            let value_to_add = (0..die_roll_count)
                                .map(|_| rng.gen_range(1..=die_side_count))
                                .sum::<i32>();

                            match operation {
                                Operation::Add => total += value_to_add,
                                Operation::Subtract => total -= value_to_add,
                            }

                            state = State::NewTerm;

                            match char {
                                '+' => operation = Operation::Add,
                                '-' => operation = Operation::Subtract,
                                ' ' => {}
                                _ => {
                                    return Err(Error::ParseError(format!(
                                        "Unexpected character when parsing die type: {}",
                                        char
                                    )))
                                }
                            }
                        }
                        Err(_) => {
                            return Err(Error::ParseError(format!(
                                "Failed to parse die type: {}",
                                buffer
                            )));
                        }
                    }
                }
                _ => {
                    return Err(Error::ParseError(format!(
                        "Unexpected character when parsing die type: {}",
                        char
                    )))
                }
            },
        }
    }

    Ok(total)
}

/// Parse the dice roll string by splitting the expression into individual semantic pieces and processing them
pub fn parse_roll_with_string_splits(input: &str) -> Result<i32, Error> {
    let input = input.replacen(' ', "", input.len());
    let input_split_by_operations = input.split_inclusive(['+', '-']).collect::<Vec<&str>>();
    // println!("{:?}", x);

    let mut total = 0;
    let mut operation = Operation::Add;
    let mut rng = rand::thread_rng();
    let total_term_count = input_split_by_operations.len();

    for (index, term) in input_split_by_operations.iter().enumerate() {
        match term.find('d') {
            Some(_) => {
                let parts = term
                    .split(['d', '+', '-'])
                    .filter(|x| !x.is_empty())
                    .collect::<Vec<&str>>();
                // println!("{:?}", parts);
                let die_count = match parts.len() {
                    1 => 1,
                    _ => parts[0].parse::<i32>().unwrap(),
                };
                let die_side_count = parts[parts.len() - 1].parse::<i32>().unwrap();
                let value_to_add = (0..die_count)
                    .map(|_| rng.gen_range(1..=die_side_count))
                    .sum::<i32>();
                match operation {
                    Operation::Add => total += value_to_add,
                    Operation::Subtract => total -= value_to_add,
                }
            }
            None => {
                let scalar = term.parse::<i32>().unwrap();
                match operation {
                    Operation::Add => total += scalar,
                    Operation::Subtract => total -= scalar,
                }
            }
        }

        let last_character = term.chars().next_back(); // As the input is split on operation characters, the last character of each non-final term is the operation character
        if index < total_term_count - 1 {
            operation = match last_character {
                Some('+') => Operation::Add,
                Some('-') => Operation::Subtract,
                _ => return Err(Error::ParseError(format!("Expected operation character at the end of term: {}", term))),
            };
        }
    }

    Ok(total)
}
