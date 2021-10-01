use rand;
use std::cmp::Ordering;
use std::io;

fn guess_game<T>(goes: u8) -> bool
where
    T: std::fmt::Display + std::str::FromStr + Ord,
    rand::distributions::Standard: rand::distributions::Distribution<T>,
{
    let target: T = rand::random();
    let mut tries: u8 = 0;

    while tries < goes {
        println!(
            "Please input your guess (you have {} goes left).",
            goes - tries
        );

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input!");

        match guess.trim().parse::<T>() {
            Ok(num) => {
                match num.cmp(&target) {
                    Ordering::Less => println!("Too small!"),
                    Ordering::Greater => println!("Too big!"),
                    Ordering::Equal => {
                        println!("You win!");
                        return true;
                    }
                }

                tries += 1;
            }
            Err(_) => {
                println!("Your input '{}' is not an integer", guess);
            }
        }
    }

    println!("You lose! The target was {}.", target);
    return false;
}

fn main() {
    println!("Welcome to my guessing game...");
    guess_game::<u8>(7);
}
