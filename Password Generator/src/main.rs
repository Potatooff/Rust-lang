// Crates
use std::io;
use colored::*;
use rand::Rng;

fn main() {
    println!("Please input your password length: ");
    let again: i8 = 1;
    while again == 1 {
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
        let password: String = generate_password(input);
        println!("Password: {}", password.green());

        println!("Do you want to generate another password?\n1- Yes\n2- No");
        let mut h_again: String = String::new();
        io::stdin()
            .read_line(&mut h_again)
            .expect("Failed to read user input");

        let h_again: i8 = match h_again.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
        if h_again == 2 {
            break;
        }
        println!("Please input your password length: ");
    }
    


}


fn generate_password(length: i32) -> String{
    let mut password = String::new();
    let mut rng = rand::thread_rng();
    let characters_lower: &str = "abcdefghijklmnopqrstuvwxyz!@#$%^&*()_+ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";

    for _ in 0..length {
        let index = rng.gen::<usize>() % characters_lower.len();
        password.push(characters_lower.chars().nth(index).unwrap());
    }

    return password;
}



