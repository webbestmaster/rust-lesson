use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess_string = String::new();

        io::stdin().read_line(&mut guess_string)
            .expect("Failed to read line");

        let guess_number: u32 = match guess_string.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} <- is not a number!!!", guess_string);
                continue;
            }
        };

        println!("You guessed: {}", guess_number);

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
