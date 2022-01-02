use std::io::{stdin};
use colored::*;

fn main() {
    const WORD_LENGTH :usize = 5;
    let word = String::from("HELLO");
    let stdin = stdin();

    let mut guess = String::new();

    println!("{}....", String::from(word.chars().nth(0).unwrap()).green());

    while guess.ne(&word) {
        guess = String::new();
        stdin.read_line(&mut guess).expect("Could not read input");

        guess = guess.trim().into();
        if guess.len() < WORD_LENGTH {
            println!("Not enough characters");
            continue;
        }

        guess = guess[0..WORD_LENGTH].to_uppercase();

        let mut matches: [i8; WORD_LENGTH] = [0; WORD_LENGTH];

        for (i, char) in guess.chars().enumerate() {
            let exact_match = char.eq(&word.chars().nth(i).unwrap());

            if exact_match {
                matches[i] = 2;
            }
        }

        for (i, char) in guess.chars().enumerate() {
            if matches[i] != 0 {
                // Stop checking if we've already found an exact match
                continue;
            }

            let positions: Vec<usize> = word.match_indices(char).map(|(position, _str)| {
                position
            }).collect();

            for position in positions {
                if matches[position] != 0 {
                    // Stop if we've already found a match (full or partial)
                    break;
                }

                matches[i] = 1;
            }
        }

        for (i, char) in guess.chars().enumerate() {
            let print_char = match matches[i] {
                1 => char.to_string().yellow(),
                2 => char.to_string().green(),
                _ => char.to_string().normal(),
            };

            print!("{}", print_char);
        }

        println!()
    }

    println!("Correct! Well done!")
}
