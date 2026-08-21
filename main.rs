use std::io;

fn main() {
    let mut iter = io::stdin().lines();
    let num: i32 = iter.next().unwrap().unwrap().parse().unwrap();

    // if num % 15 == 0 {
    //     println!("FizzBuzz");
    // } else if num % 3 == 0 {
    //     println!("Fizz");
    // } else if num % 5 == 0 {
    //     println!("Buzz");
    // } else {
    //     println!("{num}");
    // }

    let result = match (num % 3 == 0, num % 5 == 0) {
        (true, true) => "FizzBuzz",
        (true, false) => "Fizz",
        (false, true) => "Buzz",
        (false, false) => &num.to_string(),
    };

    println!("{result}")
}
