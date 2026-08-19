use std::io::{self, BufRead};

fn main() {
    let mut iter = io::stdin().lock().lines();
    let name = iter.next().unwrap().unwrap();
    let age: i32 = iter.next().unwrap().unwrap().trim().parse().unwrap();

    println!("Hi, {}! You are {} years old.", name, age);
}
