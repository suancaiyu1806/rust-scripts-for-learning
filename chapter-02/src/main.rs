use std::f64::consts::PI;
use std::io;

fn main() {
    // 1. Hello World
    println!("Hello, world!");

    // 2. 输入输出
    println!("Please input a number:");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let number: i64 = input.trim().parse().expect("Input is not a number!");
    println!("Your number is: {:?}.", number);

    // 3. 获取 π 的第 n 位
    println!("Please input the index of the π you need:");
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let index: i64 = input.trim().parse().expect("Input is not a number!");
    let decimal: String = PI
        .to_string()
        .split('.')
        .collect::<Vec<&str>>()
        .get(1)
        .expect("Get the part of decimal error.")
        .to_string();
    let result = decimal.chars().nth((index - 1) as usize).unwrap();
    println!("No.{:?} of the π is: {:?}.", index, result);
}
