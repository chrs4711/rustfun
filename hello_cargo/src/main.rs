use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);

    loop {
        println!("Plz input your guess.");
        
        let mut guess = String::new();
        let bytes_read = io::stdin()
            .read_line(&mut guess) // read_line returns an io::Result, which is an enum.
            .expect("Failed to read line"); //  It can be "Ok" or "Err".
            // if it's an "Ok", the expect function returns the no of bytes read.
            // if it's an "Err", the expect function will crash the program.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number, idiot!");
                continue;
            }
        };
    
        println!("Bytes read: {}", bytes_read);
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            // the lines here are "arms"
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Epic Win!");
                break;
            }
        }
    }

}

