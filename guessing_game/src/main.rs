use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number game! [0-100]");
    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let rand_val = rand::thread_rng().gen_range(1,100);
    println!("Your guess: {} Your rand: {}", guess, rand_val);
    
    match 
}
