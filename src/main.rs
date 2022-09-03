use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn pause(message: &str) {
    println!("{message}");
    io::stdin()
        .read_line(&mut String::new())
        .expect("Unable to use input!");
}

fn main() {
    pause("Press Enter when you are ready to play a game.");

    println!("Guest the number! (1-100)");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    pause("Press Enter To exit.");
}
