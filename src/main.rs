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

        for (i, char) in guess.chars().enumerate() {
            let print_char = match char.eq(&word.chars().nth(i).unwrap()) {
                true => String::from(char).green().bold(),
                false => String::from(char).normal(),
            };
            print!("{}", print_char);
        }
        println!();
    }

    println!("Correct! Well done!")
}
