use std::fs;

fn main() {
    let number_of_steps = 100;

    let input = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|number| char::to_digit(number));
}
