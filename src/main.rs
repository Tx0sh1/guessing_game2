use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Welcome To the guessing game!");

    loop {

        println!("Please enter your name");

        let mut name  = String::new();
        let mut lives = 4;

        io::stdin().read_line(&mut name).expect("Failed to read line");

        println!("Hi, {name} welcome to the guessing game");

        let secret_number = rand::rng().random_range(1..=100);
        println!("secret numer is: {secret_number}");

        loop {
            let mut guess = String::new();
            println!("Enter your guess:");

            io::stdin().read_line(&mut guess).expect("failed");
            let result: i32 = guess.trim().parse().expect("Enter a number");

            match result.cmp(&secret_number) {
                Ordering::Less => {

                    println!("Too small");
                    lives -= 1;
                    println!("lives left: {lives}");
                    if lives == 0 {
                        println!("Game Over, You are Out Of Lives");
                        break;
                    }
                },
                Ordering::Equal => {
                    println!("You win");
                    break;
                },
                Ordering::Greater => {
                    println!("Too high");
                    lives -= 1;
                    println!("lives left: {lives}");
                    if lives == 0 {
                        println!("Game Over!, you are out of lives");
                        break;
                    }
                }
            }
        }

        println!("would you like to play again? yes/no");
        let mut replay = String::new();
        io::stdin().read_line(&mut replay).expect("failed to read the line");
        let stripped = replay.trim();
        if stripped != "yes" {
            println!("Thank you for playing the game!");
            break
        }
    }

}
