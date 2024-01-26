use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    //Practice code
    let x = 5;
    let y = 10;
    println!("x = {x} and y + 2 = {}", y + 2);

    //Guess the number game starts from here
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        //mutable
        //(::new line indicates that new is an associated function)
        //(line has created a mutable variable that is currently bound to a new, empty instance of a String)

        io::stdin()
            .read_line(&mut guess) // & indicates that this argument is a reference
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, //The underscore is a catchfall value , continue tells the program to go to the next iteration of the loop
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            //cmp here compares the two values
            //match is like when() in kotlin
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
