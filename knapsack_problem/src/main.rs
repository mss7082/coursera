use std::collections::HashMap;
use std::{cmp, io};

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let mut capacity = 0;
    let mut n: i64 = 0;
    let mut items: Vec<(i64, i64)> = vec![];

    if let Some(i) = iter.next() {
        n = i.parse::<i64>().unwrap();
    }
    if let Some(i) = iter.next() {
        capacity = i.parse::<i64>().unwrap();
    }

    for _ in 0..n {
        let mut buf2 = String::new();
        io::stdin().read_line(&mut buf2).unwrap();
        let mut iter2 = buf2.split_whitespace();
        let mut value: i64 = 0;
        let mut weight: i64 = 0;

        if let Some(i) = iter2.next() {
            // println! {"value - {}", i}
            value = i.parse().unwrap();
        }
        if let Some(i) = iter2.next() {
            // println! {"weight - {}", i}
            weight = i.parse().unwrap();
        }

        items.push((value, weight));
    }
    println!("{:.4}", get_value(capacity, items));
    // println!("{:.4}", get_value(10, vec![(500, 30)]));
}

fn get_value(mut capacity: i64, mut items: Vec<(i64, i64)>) -> f64 {
    let mut value: f64 = 0.0;
    let mut value_weight = vec![];
    let mut items_value_ratio_to_weight = HashMap::new();

    for i in 0..items.len() {
        let value_weight_ratio: f64 = items[i].0 as f64 / items[i].1 as f64;
        value_weight.push(value_weight_ratio);
        items_value_ratio_to_weight.insert(value_weight_ratio.to_string(), items[i].1);
    }

    // println!("UnSorted value weight - {:?}", value_weight);
    value_weight.sort_by(|a, b| b.partial_cmp(a).unwrap());
    // println!("Sorted value weight - {:?}", value_weight);

    for i in 0..value_weight.len() {
        if capacity == 0 {
            return value;
        }

        let mut item_weight: i64 = 0;

        if let Some(w) = items_value_ratio_to_weight.get(&value_weight[i].to_string()) {
            item_weight = *w;
        }

        let weight_taken = cmp::min(item_weight, capacity);
        value += weight_taken as f64 * value_weight[i];

        items[i].1 -= weight_taken;
        capacity -= weight_taken;
    }

    value
}
