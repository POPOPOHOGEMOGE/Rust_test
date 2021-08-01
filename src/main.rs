use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("The secret number is: {}", secret_number);

    loop {
        println!("\nInput your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);

        let message_less = "Too small!";
        let message_greater = "Too big!";
        let message_equal = "You win!";
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", message_less.red()), 
            Ordering::Greater => println!("{}", message_greater.blue()),
            Ordering::Equal => {
                println!("{}", message_equal.yellow());
                break;
            }
        }
    }
}
