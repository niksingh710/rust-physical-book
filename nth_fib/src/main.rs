use std::io::{self, Write};

fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    print!("Enter nth number: ");

    io::stdout()
        .flush()
        .expect("Some Error Occured while flushing!");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Some Error Occured while reading!");

    let input: u32 = input.trim().parse().expect("Please type a number!");

    let result = fibonacci(input);
    println!("Fibonacci of {} is {}", input, result);
}
