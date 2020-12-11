use rand::Rng; // Trait that we need for gen_range to get defined. DO NOT REMOVE.
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing game!!");
    let secret: u32 = rand::thread_rng().gen_range(1, 101);
    loop {
        let mut guess = String::new();
        println!("Guess the number:");
        io::stdin()
            .read_line(&mut guess)
            .expect("couldn't get the guess");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("Got em!");
                break;
            }
        }
    }
}
