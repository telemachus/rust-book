use std::io;
use std::io::Write;

fn main() {
    print!("What number in the Fibonacci sequence do you want to see? ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut wanted = String::new();
    io::stdin()
        .read_line(&mut wanted)
        .expect("Failed to read line");
    let wanted = wanted.trim();
    let wanted: u32 = match wanted.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("You entered '{wanted}'. Try again with a number.");
            std::process::exit(1);
        }
    };
    show_fib_for(wanted)
}

fn show_fib_for(wanted: u32) {
    if wanted < 2 {
        println!("Number {wanted} in the Fibonacci sequence is {wanted}.");
        return;
    }
    let mut fib: u32 = 1;
    let mut fib_prev: u32 = 1;
    for _ in 2..wanted {
        (fib_prev, fib) = (fib, fib_prev + fib);
    }
    println!("Number {wanted} in the Fibonacci sequence is {fib}.");
}
