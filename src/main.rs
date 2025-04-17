use std::io;

fn main() {
    println!("Guess the number");
    println!("Please input your guess.");

    // Get user input
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");  // handles potential failuer.

    // Display guess
    println!("You guessed: {}", guess);
}