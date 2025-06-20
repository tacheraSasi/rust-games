use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number");

    println!("Please enter you guess");

    let rand_number = rand::rng().random_range(1..=100);
    let mut _output_msg = "You have guessed {}";

    let err_msg = "Something went wrong. {}";
    loop {
         
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect(err_msg);
        
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
        println!("You have guessed {}", guess);
        // if guess == rand_number {
        //     println!("You have guessed right, You win!");
        //     break;
        // }
    
        match guess.cmp(&rand_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
