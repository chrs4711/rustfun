use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);

    println!("Plz input your guess.");
    let mut guess = String::new();

    let bytes_read = io::stdin()
        .read_line(&mut guess) // read_line returns an io::Result, which is an enum.
        .expect("Failed to read line"); //  It can be "Ok" or "Err".
        // if it's an "Ok", the expect function returns the no of bytes read.
        // if it's an "Err", the expect function will crash the program.

    println!("Bytes read: {}", bytes_read);

    println!("You guessed: {}", guess);
}
