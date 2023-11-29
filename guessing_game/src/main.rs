use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please type in your guess...");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("you guessed: {guess}");
}
