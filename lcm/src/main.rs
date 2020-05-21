use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let mut a: usize = 0;
    let mut b: usize = 0;
    if let Some(i) = iter.next() {
        a = i.parse().unwrap();
    }
    if let Some(i) = iter.next() {
        b = i.parse().unwrap();
    }
    // println!("{}", naive_lcm(761457, 614573))
    println!("{}", better_lcm(a, b))
}

// fn naive_lcm(a: usize, b: usize) -> usize {
//     let mut z: usize;
//     let lcm: usize;
//     if a > b {
//         z = a;
//     } else {
//         z = b;
//     }

//     loop {
//         if z % a == 0 && z % b == 0 {
//             lcm = z;
//             break;
//         }
//         z += 1;
//     }
//     lcm
// }

fn better_lcm(a: usize, b: usize) -> usize {
    let mut z: usize;
    let init: usize;
    let lcm: usize;
    if a > b {
        z = a;
        init = z;
    } else {
        z = b;
        init = z;
    }

    loop {
        if z % a == 0 && z % b == 0 {
            lcm = z;
            break;
        }
        z += init;
    }
    lcm
}
