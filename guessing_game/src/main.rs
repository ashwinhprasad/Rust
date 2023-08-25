extern crate rand;


use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    
    let secret_number: u32 = rand::thread_rng().gen_range(1..100);

    loop {

        let mut guess = String::new();
        
        println!("Enter your guess : ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Error while reading input");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Found the Guess");
                break;
            },
            Ordering::Less => println!("A little higher"),
            Ordering::Greater => println!("A little lower")
        }
    }

}
