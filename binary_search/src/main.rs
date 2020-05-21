use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let raw_iter = buf.split_whitespace();
    let mut input: Vec<usize> = raw_iter.map(|x| x.parse::<usize>().unwrap()).collect();
    // println!("input - {:?}", input);

    let mut buf2 = String::new();
    stdin().read_line(&mut buf2).unwrap();
    let raw_iter2 = buf2.split_whitespace();
    let mut keys: Vec<usize> = raw_iter2.map(|x| x.parse::<usize>().unwrap()).collect();
    input.remove(0);
    keys.remove(0);
    let low = 0;
    let high = input.len() - 1;
    let mut output = String::new();
    // println!("keys - {:?}", keys);
    // println!("input - {:?}", input);

    // println!("{:?}", linear_search(input, keys));
    for key in keys {
        let result = binary_search(input.clone(), low, high, key);
        output.push_str(&result.to_string());
        output.push_str(" ");
        // println!("{}", binary_search(input.clone(), low, high, key));
    }
    println!("{}", output);
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
