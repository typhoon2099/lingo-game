// use colored::*;

fn main() {
    let word = String::from("HELLO");

    // print!("{}", "H".green());
    // print!("E");
    // print!("L");
    // print!("L");
    // print!("O");
    // println!();

    let mut guess = String::new();

    println!("H....");

    while guess.ne(&word) {
        if guess.ne("") {
            println!("Nope");
        }
        guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("Could not read input");
        guess = guess[0..5].into();
        println!("Guess: {}", guess);
    }

    println!("Correct! Well done!")
}
