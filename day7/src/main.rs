use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap().replace("\n", "");
    let crabs: Vec<u32> = input
        .split(",")
        .map(|item| item.parse::<u32>().unwrap())
        .collect();

    let mut best_fuel = 100000000;
    let mut position = 0;
    let mut i = 0;
    while i < crabs.len() {
        let mut fuel = 0;
        for crab in crabs.clone() {
            let diff = (crab as i64 - i as i64).abs();
            fuel += (diff * (diff + 1)) / 2;
        }
        if fuel < best_fuel {
            best_fuel = fuel;
            position = i;
        }

        i += 1;
    }
    println!("Pos: {}, Total Fuel Used: {}", position, best_fuel);
}
