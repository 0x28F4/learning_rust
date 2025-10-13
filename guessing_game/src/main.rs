use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("Guess the number!");

    let secret_num = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        
        stdin().read_line(&mut guess).expect("Failed to read line");

        let guess = match u32::from_str_radix(&guess.trim(), 16) {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("You guessed: {guess}{secret_num}");
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("smol"),
            Ordering::Equal => {println!("got it!"); break;},
            Ordering::Greater => println!("too big boi"),
        }
    }
}
