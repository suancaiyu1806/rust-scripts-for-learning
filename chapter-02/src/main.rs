use std::io;

fn main() {
    println!("Hello, world!");
    println!("Please input a number:");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("Your raw input is: {:?}.", input);
    let number: i64 = input.trim().parse().expect("Input is not a number!");
    println!("Your number is: {:?}.", number);
}
