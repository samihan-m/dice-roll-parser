use parsing::{parse_roll_with_state_machine, parse_roll_with_regex, parse_roll_with_string_splits};

#[macro_use]
extern crate timeit;

pub mod parsing;

fn main() {
    println!("State machine parse results:");
    timeit!({
        parse_roll_with_state_machine("d20 + 5");
    });
    timeit!({
        parse_roll_with_state_machine("2d10 + 1d8 - 1d4 + 3");
    });

    println!("String split parse results:"); 
    timeit!({
        parse_roll_with_string_splits("d20 + 5");
    });
    timeit!({
        parse_roll_with_string_splits("2d10 + 1d8 - 1d4 + 3");
    });

    println!("Regex parse results:");
    timeit!({
        parse_roll_with_regex("d20 + 5");
    });
    timeit!({
        parse_roll_with_regex("2d10 + 1d8 - 1d4 + 3");
    });
}
