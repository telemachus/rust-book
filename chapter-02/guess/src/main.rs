use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");

    loop {
        print!("Please input your guess: ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim() {
            "q" | "quit" => {
                println!("Thanks for playing!");
                std::process::exit(0);
            }
            _ => (),
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("{} is correct: congratulations!", guess);
                break;
            }
        }
    }
}
