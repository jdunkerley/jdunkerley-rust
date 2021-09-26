use std::io;
use rand;

fn guess_game<T>(goes: u8) -> bool 
where 
    T: std::cmp::PartialOrd + std::fmt::Display + std::str::FromStr,
    rand::distributions::Standard: rand::distributions::Distribution<T>
{
    let target: T = rand::random();
    let mut tries: u8 = 0;

    while tries < goes {
        println!("Please input your guess (you have {} goes left).", goes - tries);
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input!");

        match guess.trim().parse::<T>() {
            Ok(num) => {
                if num == target {
                    println!("You win!");
                    return true;
                } else if num < target {
                    println!("Too Low!");
                } else {
                    println!("Too High!");
                }
                tries += 1;
            },
            Err(_) => {
                println!("Your input '{}' is not an integer", guess);
            }
        }
    }

    println!("You lose! The target was {}." , target);
    return false;
}

fn main() {
    println!("Welcome to my guessing game...");
    println!("You have 5 tries to guess the number I'm thinking of.");
    guess_game::<u8>(5);
}
