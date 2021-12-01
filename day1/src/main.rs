//use std::env;
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use std::fs;

fn main() {
    let mut first: u32;
    let mut second: u32;
    let mut third: u32;
    let mut fourth: u32;
    let mut previous: u32;
    let mut current: u32;
    let mut total_increased: u16 = 0;

    let input =
        fs::read_to_string("input.txt").expect("Something went wrong reading the input file");
    let mut lines = input.lines().peekable();
    first = lines.next().unwrap().parse::<u32>().unwrap();
    second = lines.next().unwrap().parse::<u32>().unwrap();
    third = lines.next().unwrap().parse::<u32>().unwrap();
    fourth = lines.next().unwrap().parse::<u32>().unwrap();
    previous = first + second + third;
    current = second + third + fourth;
    if current > previous {
        total_increased = total_increased + 1;
    }
    while lines.peek() != None {
        previous = current;
        first = second;
        second = third;
        third = fourth;
        fourth = lines.next().unwrap().parse::<u32>().unwrap();
        current = second + third + fourth;
        if current > previous {
            total_increased = total_increased + 1;
        }
    }
    println!("Total increased: {}", total_increased)
}
