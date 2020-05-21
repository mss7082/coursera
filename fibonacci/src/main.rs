// use rand::distributions::{Distribution, Uniform};
use std::io;

// fn stress_test() {
//     loop {
//         let mut rng = rand::thread_rng();
//         let number = Uniform::from(2..21);
//         let n = number.sample(&mut rng);

//         for i in 2..n {
//             let naive_result = calc_fibo_naive(i);
//             let euclid_result = calc_fibo_euclid(i);
//             if naive_result == euclid_result {
//                 println!("fibo({}) - OK", i);
//             } else {
//                 println!(
//                     "Wrong answer - Naive = {}, Euclid = {}",
//                     naive_result, euclid_result
//                 );
//                 break;
//             }
//         }
//     }
// }

fn main() {
    let n = get_input();
    println!("{}", calc_fibo_euclid(n));
}

fn get_input() -> usize {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let input: usize = buf.trim_end().parse().unwrap();

    input
}

// fn calc_fibo_naive(n: usize) -> usize {
//     if n <= 1 {
//         return n;
//     }

//     calc_fibo_naive(n - 1) + calc_fibo_naive(n - 2)
// }

fn calc_fibo_euclid(n: usize) -> usize {
    let mut fibo_result = vec![0; n + 1];

    if n <= 1 {
        return n;
    } else {
        fibo_result[0] = 0;
        fibo_result[1] = 1;
        for i in 2..=n {
            fibo_result[i] = fibo_result[i - 1] + fibo_result[i - 2];
        }
    }

    fibo_result[n]
}
