use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let mut a: i64 = 0;
    let mut b: i64 = 0;
    if let Some(i) = iter.next() {
        a = i.parse().unwrap();
    }
    if let Some(i) = iter.next() {
        b = i.parse().unwrap();
    }

    println!("{}", euclidean_gcd(a, b));
}

fn euclidean_gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    let a_prime = &a % &b;
    euclidean_gcd(b, a_prime)
}
