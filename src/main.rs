use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to the guessing game, enter a number from 1 to 100");
    //get user input
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("coudn't process your guess");
    println!("you guessed: {}", guess);

    //generate random number
    // let random_number = rand::thread_rng().gen_range(1..=100);
    println!("Random number is {}", rand::thread_rng().gen_range(1..=100))
}

//get user input
//generate random number
//assert or check if the random numbers are equal
