use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt");
    let mut lines = input.iter();
    #[allow(unused_mut)]
    let mut grid = [[0u8; 9]; 9];
    println!("{:?}", grid)
}
