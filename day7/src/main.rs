use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap().replace("\n", "");
    let crabs: Vec<u16> = input
        .split(",")
        .map(|item| item.parse::<u16>().unwrap())
        .collect();
    let mean: f32 = crabs.iter().sum::<u16>() as f32 / crabs.len() as f32;
    let frequencies = crabs.iter().fold(HashMap::new(), |mut freqs, value| {
        *freqs.entry(value).or_insert(0) += 1;
        freqs
    });

    let mode = frequencies
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| *value)
        .unwrap();

    println!("mean:{} mode: {}", mean, mode);
}
