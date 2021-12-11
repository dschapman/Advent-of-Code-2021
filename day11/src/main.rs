use std::fs;

struct OctoGrid(Vec<Vec<32>>)

fn main() {
    let _number_of_steps = 100;

    let input: Vec<Vec<u32>> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|number| number.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    println!("{:?}", input)
}
