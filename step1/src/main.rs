// Crates
use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Welcome to a Potato Guessing Game");
    println!("Please input your guess: ");
    let secret_number: i32 = rand::thread_rng().gen_range(1, 101);

    println!("NPC NUMBER IS {}", secret_number);
    loop {
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "You win".green());
                break;
            }
        }
    }
}