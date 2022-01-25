use colored::*;
use std::io::stdin;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "4", help = "Play a 4 letter word game")]
    four: bool,
}

struct GuessCharacter {
    char: char,
    position: Option<usize>,
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

        let mut match_data: Vec<GuessCharacter> = vec![];

        for char in guess.chars() {
            match_data.push(GuessCharacter{char, position: None});
        };

        let mut matched = vec![false; word_length];

        for (i, character_match) in match_data.iter_mut().enumerate() {
            let exact_match = character_match.char.eq(&word.chars().nth(i).unwrap());

            if exact_match {
                matched[i] = true;
                character_match.position = Some(i);
                continue;
            }

            // Positions of matching characters
            let positions: Vec<usize> = word
                .match_indices(character_match.char)
                .map(|(position, _str)| position)
                .collect();

            for position in positions {
                // Skip if we've already found a match for the current position
                if matched[position] {
                    continue;
                }

                matched[position] = true;
                character_match.position = Some(position);

                break; // We only want the first proper match
            }
        }

        for (i, character_match) in match_data.iter().enumerate() {
            let character = match character_match.position {
                Some(position) if position == i => character_match.char.to_string().green(),
                Some(_position) => character_match.char.to_string().yellow(),
                None => character_match.char.to_string().normal(),
            };


            print!("{}", character);
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
