use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let n: i32 = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let result = square(n);
    println!("{}", result);
}

fn square(n: i32) -> i32 {
    n * n
}
