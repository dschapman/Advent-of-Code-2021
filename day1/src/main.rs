//use std::env;
use dhat::{Dhat, DhatAlloc};
use std::fs;

#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;
fn main() {
    let _dhat = Dhat::start_heap_profiling();
    let first: u16;
    let mut second: u16;
    let mut third: u16;
    let mut fourth: u16;
    let mut previous: u16;
    let mut current: u16;
    let mut total_increased: u16 = 0;

    let input =
        fs::read_to_string("input.txt").expect("Something went wrong reading the input file");
    let mut lines = input.lines().peekable();
    first = lines.next().unwrap().parse::<u16>().unwrap();
    second = lines.next().unwrap().parse::<u16>().unwrap();
    third = lines.next().unwrap().parse::<u16>().unwrap();
    fourth = lines.next().unwrap().parse::<u16>().unwrap();
    previous = first + second + third;
    current = second + third + fourth;
    if current > previous {
        total_increased = total_increased + 1;
    }
    while lines.peek() != None {
        previous = current;
        second = third;
        third = fourth;
        fourth = lines.next().unwrap().parse::<u16>().unwrap();
        current = second + third + fourth;
        if current > previous {
            total_increased = total_increased + 1;
        }
    }
    println!("Total increased: {}", total_increased)
}
