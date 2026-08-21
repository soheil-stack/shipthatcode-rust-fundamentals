use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let num: i32 = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse()
        .unwrap();

    // if num % 15 == 0 {
    //     println!("FizzBuzz");
    // } else if num % 3 == 0 {
    //     println!("Fizz");
    // } else if num % 5 == 0 {
    //     println!("Buzz");
    // } else {
    //     println!("{num}");
    // }

    match (num % 3 == 0, num % 5 == 0) {
        (true, true) => println!("FizzBuzz"),
        (true, false) => println!("Fizz"),
        (false, true) => println!("Buzz"),
        (false, false) => println!("{num}"),
    };
}
