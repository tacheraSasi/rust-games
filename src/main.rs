use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number");

    println!("Please enter you guess");

    let mut guess = String::new();
    let rand_number = rand::rng().random_range(1..=100);
    let mut _output_msg = "You have guessed {}";
    let mut is_match: bool = false;

    let err_msg = "Something went wrong. {}";
    while(!is_match){
        io::stdin().read_line(&mut guess).expect(err_msg);
        
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
        println!("You have guessed {}", guess);
        if(guess == rand_number){
            println!("You have guessed right, You win!");
            is_match = true;
        }
    
        match guess.cmp(&rand_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => println!("You win!")
        }
    }
}
