use std::{io, num};

use rand::Rng;
use std::cmp::Ordering;
fn main() {

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut input = String::new();
        println!("Please enetr a number between 1 and 100:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
    
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }

}
