use std::io;

fn get_fibonacci_last_digit(n: usize) -> usize {
    let mut first = 0;
    let mut second = 1;

    let mut result: usize = 0;

    for _ in 2..=n {
        result = (first + second) % 10;
        first = second;
        second = result;
    }

    result
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();
    println!("{}", get_fibonacci_last_digit(n));
}
