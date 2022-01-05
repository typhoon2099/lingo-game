use std::io::stdin;
use colored::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "4", help = "Play a 4 letter word game")]
    four: bool,
}


fn main() {
    let opt = Opt::from_args();

    let mut word_length = 5;
    if opt.four {
        word_length = 4;
    }

    let word = get_word(word_length);
    let stdin = stdin();

    let mut guess = String::new();

    print!("{}", String::from(word.chars().nth(0).unwrap()).green());
    for _num in 1..word_length {
        print!(".");
    }
    println!();

    while guess.ne(&word) {
        guess = String::new();
        stdin.read_line(&mut guess).expect("Could not read input");

        guess = guess.trim().into();
        if guess.len() < word_length {
            println!("Not enough characters");
            continue;
        }

        guess = guess[0..word_length].to_uppercase();

        let mut matches = vec!(0; word_length);
        let mut matched = vec!(false; word_length);

        for (i, char) in guess.chars().enumerate() {
            let exact_match = char.eq(&word.chars().nth(i).unwrap());

            if exact_match {
                matches[i] = 2;
                matched[i] = true;
            }
        }

        for (i, char) in guess.chars().enumerate() {
            // Stop checking the current character if we've already found an exact match
            if matches[i] != 0 { continue; }

            // Positions of matching characters
            let positions: Vec<usize> = word
                .match_indices(char)
                .map(|(position, _str)| { position })
                .collect();

            for position in positions {
                // Skip if we've already found a match for the current position
                if matched[position] { continue; }

                matches[i] = 1;
                matched[position] = true;

                break; // We only want the first proper match
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

fn get_word(length: usize) -> String {
    let four = String::from_utf8(include_bytes!("../4.txt").to_vec()).unwrap();
    let five = String::from_utf8(include_bytes!("../5.txt").to_vec()).unwrap();

    let words: Vec<&str>;
    if length == 5 {
        words = five.split("\n").collect();
    } else {
        words = four.split("\n").collect();
    }

    let position = (rand::random::<f32>() * words.len() as f32) as usize;

    String::from(words[position])
}
