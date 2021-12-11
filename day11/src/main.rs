use std::fs;

#[derive(Debug)]
struct OctoGrid(Vec<Vec<u32>>);

fn main() {
    let _number_of_steps = 100;

    let input: OctoGrid = Iterator::collect::<_>(
        fs::read_to_string("input.txt")
            .unwrap()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|number| number.to_digit(10).unwrap())
                    .collect()
            }),
    );
    println!("{:?}", input)
}
