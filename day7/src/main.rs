use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap().replace("\n", "");
    let crabs: Vec<u32> = input
        .split(",")
        .map(|item| item.parse::<u32>().unwrap())
        .collect();
    let number_of_crabs = crabs.len();
    let mean: f64 = crabs.iter().sum::<u32>() as f64 / crabs.len() as f64;
    let mut sum_of_squares: f64 = 0.0;
    for crab in crabs.clone() {
        sum_of_squares += (crab as f64 - mean).abs().powi(2)
    }
    let std_dev = (sum_of_squares / number_of_crabs as f64).sqrt();
    let new_mean = crabs
        .clone()
        .iter()
        .filter(|crab| crab as f64 < mean + std_dev && crab as f64 > mean - std_dev)
        .sum();
    println!("mean:{} standad deviation: {}", mean, std_dev);
}
