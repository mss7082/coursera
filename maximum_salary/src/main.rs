fn main() {
    largest_number(vec![21, 2]);
}

fn largest_number(mut digits: Vec<i64>) {
    let mut answer = String::new();
    digits.sort_by(|a, b| a.cmp(b));
    while !digits.is_empty() {
        let mut max_digit = 0;
        for digit in digits.iter() {
            if greater_or_equal(*digit, max_digit) {
                max_digit = *digit;
            }
        }
        answer.push_str(&max_digit.to_string());
        digits.pop();
    }
    println!("{}", answer);
}

fn greater_or_equal(digit: i64, max_digit: i64) -> bool {
    let mut a = String::new();
    let mut b = String::new();

    a.push_str(&digit.to_string());
    a.push_str(&max_digit.to_string());
    b.push_str(&max_digit.to_string());
    b.push_str(&digit.to_string());

    println!("a - {}", a);
    println!("b - {}", b);

    a.parse::<i64>().unwrap() >= b.parse::<i64>().unwrap()
}
