use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let m = buf.trim_end().parse::<usize>().unwrap();
    println!("{}", naive_money_change(m));
}

fn naive_money_change(mut m: usize) -> usize {
    let mut coins = vec![1, 5, 10];
    let mut counter: usize = 0;

    loop {
        while m < coins[coins.len() - 1] {
            coins.pop();
        }
        let current_largest_coin = coins[coins.len() - 1];
        let remain = m - current_largest_coin;
        counter += 1;
        if remain == 0 {
            return counter;
        }
        m = remain;
    }
}
