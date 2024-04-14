use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    let mut min: u32 = 0;
    let mut max: u32 = 100;
    let number = rand::thread_rng().gen_range(min..=max);

    loop {
        print!("Enter Your Choice ({min}-{max}): ");
        io::stdout()
            .flush()
            .expect("Some Error Occured while flushing!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("OS failed in stdin");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess > max || guess < min {
            println!("Your maximum range is ({min}-{max})");
            continue;
        }
        match guess.cmp(&number) {
            Ordering::Less => {
                println!("Your Choice is Less!");
                min = guess;
            }
            Ordering::Greater => {
                println!("Your Choice is Big!");
                max = guess;
            }
            Ordering::Equal => {
                println!("You Won!");
                break;
            }
        }
    }
}
