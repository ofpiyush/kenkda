use std::io;
use rand::Rng;
use rand; // unnecessary habit that I have from other languages
use std::cmp::Ordering;

fn main() {
    println!("Guess the number between 1-10!");
    let secret: u32 = rand::thread_rng().gen_range(1,11);
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Reading from stdin broke.");
    let guess: u32 = guess.trim().parse().expect("I neeeded a number");
    match guess.cmp(&secret) {
        Ordering::Less => println!("Too low!"),
        Ordering::Greater => println!("Too high!"),
        Ordering::Equal => println!("Bingo bango!")
    }
}
