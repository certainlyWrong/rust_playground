use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..=100);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        let result = io::stdin().read_line(&mut guess);

        match result {
            Ok(_) => {
                let guess: u32 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a number");
                        continue;
                    }
                };
                println!("You guessed: {}", guess);

                match guess.cmp(&random_number) {
                    Ordering::Less => println!("Too small!"),
                    Ordering::Greater => println!("Too big!"),
                    Ordering::Equal => {
                        println!("You win");
                        break;
                    }
                }
            }
            Err(_) => {
                println!("Error reading input");
            }
        }
    }
}
