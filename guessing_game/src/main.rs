// Prelude - bring libraries into scope
use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    println!("Please enter your guess: ");

    let mut guess = String::new(); //::new() means that new() is an associated function

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
        Ordering::Less => println!("Too small!"),
    }
}
