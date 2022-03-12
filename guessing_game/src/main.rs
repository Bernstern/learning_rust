use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is: {}", secret);

    // Loop until the guess is correct!
    loop {
        let mut guess = String::new();

        // Try swapping the line for this one, cargo check freaks out because readline cannot borrow it
        // let guess = String::new();

        // Standard syntax is to break up chained method calls onto new lines (not sure how I feel about this)
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line...");

        // Try to cast guess to a i32 and use a match statement to handle errors (OCaml vibes...)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("Invalid, please enter another guess:"); continue;},
        };

        println!("You guessed: {}", guess);

        // Compare the guess using a match expression which returns an Ordering Enum which we match an arm to
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {println!("You win!"); break;}
        }
    }
}
