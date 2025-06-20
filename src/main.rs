use std::io;

fn main() {
    println!("Guess a number");

    println!("Please enter you guess");

    let mut guess = String::new();
    let mut _output_msg = "You have guessed {}";

    let err_msg = "Something went wrong. {}";
    io::stdin().read_line(&mut guess).expect(err_msg);

    println!("You have guessed {}", guess)
}
