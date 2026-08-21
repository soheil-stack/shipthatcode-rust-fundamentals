use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let n: i64 = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let mut sum: i64 = 0;
    for i in 1..=n {
        sum += i;
    }

    println!("{}", sum);
}
