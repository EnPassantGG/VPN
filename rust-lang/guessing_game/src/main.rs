use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::random_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Can reassign a variable name to a different type! (variable shadowing)
        // This 'match' also allows users to enter non-integer numbers and keep going
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // Now this doesn't have a mismatched types error
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
