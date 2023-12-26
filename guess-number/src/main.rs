use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    guessing_game();
}

fn guessing_game() {
    println!("Guess the number!");

    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    println!("Secret Number Is: {secret_number}");

    loop {
        println!("enter your number");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            // .unwrap();
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Failed to parse");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Less"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => {
                println!("Equal");
                break;
            },
        }

    }

    // println!("You guessed: {guess}");
}

#[warn(dead_code)]
fn main_() {
    println!("Hello, world!");
}
