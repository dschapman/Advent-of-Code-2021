//use std::env;
#![allow(unused_assignments)]
use std::fs;

fn main() {
    let mut current: u32;
    let mut previous: u32;
    let mut total_increased: u16 = 0;

    let input =
        fs::read_to_string("input.txt").expect("Something went wrong reading the input file");
    let mut lines = input.lines();
    //first run
    previous = lines
        .next()
        .expect("Expected string contents to be present")
        .parse::<u32>()
        .unwrap();
    current = lines
        .next()
        .expect("Expected string contents to be present")
        .parse::<u32>()
        .unwrap();
    if current > previous {
        total_increased = total_increased + 1;
    }
    for line in lines {
        previous = current;
        current = line.parse::<u32>().unwrap();
        if current > previous {
            total_increased = total_increased + 1;
        }
    }
    println!("Total times increased: {}", total_increased)
}
