// Prelude - bring libraries into scope
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please enter your guess: ");

    let mut guess = String::new(); //::new() means that new() is an associated function

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed: {guess}")
}
