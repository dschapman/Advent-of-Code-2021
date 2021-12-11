use ansi_term::Style;
use std::fmt;
use std::fs;

#[derive(Debug)]
struct OctoGrid(Vec<Vec<u32>>);

impl OctoGrid {
    fn step(&mut self) {
        for i in 0..10 {
            for j in 0..10 {
                self.0[i][j] += 1;
            }
        }
    }
}

impl fmt::Display for OctoGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..10 {
            for j in 0..10 {
                if self.0[i][j] == 0 {
                    write!(f, "{}", Style::new().bold().paint(self.0[i][j].to_string()))?;
                }
                write!(f, "{}", self.0[i][j])?
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
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
    let octo_grid: OctoGrid = OctoGrid(input);
    println!("{}", octo_grid)
}
