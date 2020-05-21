use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let raw_iter = buf.split_whitespace();

    let input: Vec<usize> = raw_iter.map(|x| x.parse::<usize>().unwrap()).collect();
    println!("input - {:?}", input);

    // for v in raw_iter {
    //     println!("v - {}", v);
    // }
    // if let Some(i) = raw_iter.next() {
    //     let n = i.parse::<i64>().unwrap();
    // }

    // let input: Vec<usize> = vec![1, 5, 8, 12, 13];
    // let key: usize = 23;
    // let high = input.len() - 1;
    // let low = 0;
    // let keys: Vec<usize> = vec![8, 1, 23, 1, 11];
    // println!("{:?}", linear_search(input, keys));
    // println!("{:?}", binary_search(input, low, high, key));
}

// fn linear_search(input: Vec<usize>, keys: Vec<usize>) -> Vec<i64> {
//     let mut idx: Vec<i64> = vec![];
//     for key in keys.iter() {
//         if let Some(j) = input.iter().position(|x| x == key) {
//             idx.push(j as i64);
//         } else {
//             idx.push(-1);
//         }
//     }

//     idx
// }

fn binary_search(input: Vec<usize>, low_idx: usize, high_idx: usize, key: usize) -> i64 {
    if high_idx < low_idx {
        return -1;
    }

    let mid_idx = low_idx + ((high_idx - low_idx) / 2);

    if key == input[mid_idx] {
        return mid_idx as i64;
    } else if key < input[mid_idx] {
        return binary_search(input, low_idx, mid_idx - 1, key);
    } else {
        return binary_search(input, mid_idx + 1, high_idx, key);
    }
}
