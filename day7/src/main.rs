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
    let trimmed_crabs: Vec<u32> = crabs
        .clone()
        .into_iter()
        .filter(|crab| (f64::from(*crab) < mean + std_dev && f64::from(*crab) > mean - std_dev))
        .collect();
    let trimmed_mean = trimmed_crabs.iter().sum::<u32>() as f64 / trimmed_crabs.len() as f64;
    println!(
        "mean:{} standard deviation: {} trimmed_mean: {}",
        mean, std_dev, trimmed_mean
    );
    let mut i = -15;
    while i < 15 {
        let mut fuel = 0;
        for crab in crabs.clone() {
            fuel += (crab as i64 - trimmed_mean.round() as i64 + i as i64).abs();
        }
        println!(
            "Pos: {}, Total Fuel Used: {}",
            trimmed_mean.round() + i as f64,
            fuel
        );

        i += 1;
    }
}
