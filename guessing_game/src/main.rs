use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Variable shadowing
        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Use Ordering enum types to match the expression
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "To less".red()),
            Ordering::Equal => {
                println!("{}", "Success!".green());
                break;
            }
            Ordering::Greater => println!("{}", "Too big".red()),
        }
    }
}
