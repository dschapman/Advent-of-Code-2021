use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let fish = input.split(",");
    println!("{:?}", fish)
}
