use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("This is a guessing game.");

    let dice = rand::thread_rng().gen_range(-10..=72);
    loop {
        println!("your guess: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("Incorrect!");

        let guess: i32 =  match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&dice) {
            Ordering::Less => println!("Teeny number"),
            Ordering::Greater => println!("Whoa there!"),
            Ordering::Equal => {
                println!("Next! Oh, uhh... you got it. Congrats.");
                break;
            }
        }
    }
}
