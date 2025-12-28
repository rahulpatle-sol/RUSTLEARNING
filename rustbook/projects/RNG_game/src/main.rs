// ===============================
// 🎯 Guessing Game — CLI Project
// ===============================

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("🔢 Welcome to the Guessing Game!");
    println!("I'm thinking of a number between 1 and 100...");

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        // Trim and parse input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Please enter a valid number.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("📉 Too low!"),
            Ordering::Greater => println!("📈 Too high!"),
            Ordering::Equal => {
                println!("🎉 You got it!");
                break;
            }
        }
    }
}
