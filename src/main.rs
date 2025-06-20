use std::io;
use rand::rng;

fn main() {
    println!("Guess a number");

    println!("Please enter you guess");

    let mut guess = String::new();
    let rand_number = rand::thread_rng().gen_range(1..=100)
    let mut _output_msg = "You have guessed {}";

    let err_msg = "Something went wrong. {}";
    io::stdin().read_line(&mut guess).expect(err_msg);

    println!("You have guessed {}", guess)
}
