use std::io;
use rand::Rng;

fn main() {
    println!("Welcome To the guessing game!");
    println!("Please enter your name");

    let mut name  = String::new();

    io::stdin().read_line(&mut name).expect("Failed to read line");

    println!("Hi, {name} welcome to the guessing game");

    let secret_number = rand::rng().random_range(1..=100);
    println!("secret numer is: {secret_number}");

    loop {
        let mut guess = String::new();
        println!("Enter your guess:");

        io::stdin().read_line(&mut guess).expect("failed");
        let result :i32 = match guess.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("invalid input. Enter a number");
                continue;
            }
        };
    }


}
