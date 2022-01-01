use std::io::{stdin};
use colored::*;

fn main() {
    const WORD_LENGTH :usize = 5;
    let word = String::from("WORLD");

    let mut guess = String::new();

    println!("{}....", String::from(word.chars().nth(0).unwrap()).green());

    while guess.ne(&word) {
        guess = String::new();
        stdin().read_line(&mut guess).expect("Could not read input");

        guess = guess.trim()[0..WORD_LENGTH].to_uppercase();

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
