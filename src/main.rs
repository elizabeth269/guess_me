use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    //generate random number
    let random_number = rand::thread_rng().gen_range(1..=100);
    // println!("Random number is {}", random_number);
    println!("Welcome to the guessing game, enter a number from 1 to 100");
    let mut count = 0;
    loop {
        // let count = count + 1;
        //get user input
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("coudn't process your guess");

        //converting our guess from string to an integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // expect("not an integer");
        println!("you guessed: {}", guess);

        //assert or check if the random numbers are equal
        match guess.cmp(&random_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
        count += 1;
    }

    println!("you guessed: {count} times")
}

//get user input
//generate random number
//assert or check if the random numbers are equal
