use dhat::{Dhat, DhatAlloc};
use std::fs;
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

#[allow(dead_code)]
fn part_1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();
    let mut x_pos: u32 = 0;
    let mut z_depth: u32 = 0;
    for line in lines {
        let mut words = line.split_whitespace();
        let direction = words.next().unwrap();
        if direction == "forward" {
            x_pos += words.next().unwrap().parse::<u32>().unwrap();
        } else if direction == "up" {
            z_depth -= words.next().unwrap().parse::<u32>().unwrap();
        } else if direction == "down" {
            z_depth += words.next().unwrap().parse::<u32>().unwrap();
        } else {
            panic!("Bad input")
        }
    }
    let result: u32 = x_pos * z_depth;
    println!("Final horizontal distance: {}", x_pos);
    println!("Final depth: {}", z_depth);
    println!("x*z: {}", result)
}

fn part_2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines();
    let mut x_pos: u32 = 0;
    let mut z_depth: u32 = 0;
    let mut aim: u32 = 0;
    for line in lines {
        let mut words = line.split_whitespace();
        let direction = words.next().unwrap();
        if direction == "forward" {
            let x = words.next().unwrap().parse::<u32>().unwrap();
            x_pos += x;
            z_depth += aim * x
        } else if direction == "up" {
            aim -= words.next().unwrap().parse::<u32>().unwrap();
        } else if direction == "down" {
            aim += words.next().unwrap().parse::<u32>().unwrap();
        } else {
            panic!("Bad input")
        }
    }
    let result: u32 = x_pos * z_depth;
    println!("Final horizontal distance: {}", x_pos);
    println!("Final depth: {}", z_depth);
    println!("x*z: {}", result)
}

fn main() {
    let _dhat = Dhat::start_heap_profiling();
    //part_1();
    part_2();
}
