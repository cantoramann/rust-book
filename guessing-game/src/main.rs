// imports
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // generate a random number between 1, 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess.");

        // * generating the string inside the loop may be costly. Yet, declaring this outside has issues due to casting below.
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!"); // expectes crash the program

        // numeric casting
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
