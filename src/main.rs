extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng

fn main() {
    println!("Guess the number!");
    println!("Enter number between 0 and 100");

    let mut guess = String::new();

    let num = rand::thread_rng().gen_range(0,100);

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read a line");

    let guess: u32 = guess.trim().parse()
        .ok()
        .expect("Please type a number!");

    println!("You entered {}", guess);

    match guess.cmp(&num) {
        Ordering::Less => println!("To small"),
        Ordering::Greater => println!("To big"),
        Ordering::Equal => println!("Equals!"),
    }

}
