fn main() {
    let stops = vec![200, 375, 550, 750];
    let distance = 950;
    let tank = 400;
    println!("{}", min_refills(stops, distance, tank));
}

fn min_refills(stops: Vec<usize>, distance: usize, tank: usize) -> i64 {
    let mut num_of_refills = 0;
    let mut current_refill = 0;

    while current_refill <= stops.len() - 1 {
        let last_refill = current_refill;

        while current_refill <= stops.len() - 1
            && (stops[current_refill + 1] - stops[last_refill] <= tank)
        {
            current_refill += 1;
        }
        if current_refill == last_refill {
            return -1;
        }
        if current_refill <= stops.len() - 1 {
            num_of_refills += 1;
        }
    }

    num_of_refills
}
