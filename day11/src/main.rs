use std::fs;

fn main() {
    let number_of_steps = 100;

    let input: Vec<Vec<u8>> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|number| number.to_digit(10).unwrap())
                .collect()
        })
        .collect();
}
