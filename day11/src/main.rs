use ansi_term::Style;
use std::fmt;
use std::fs;

#[derive(Debug)]
struct OctoGrid(Vec<Vec<u32>>, Vec<(usize, usize)>, u128);

impl OctoGrid {
    fn step(&mut self) {
        for i in 0..10 {
            for j in 0..10 {
                self.0[i][j] += 1;
            }
        }

        for i in 0..10 {
            for j in 0..10 {
                if self.0[i][j] > 9 {
                    self.flash(i, j);
                }
            }
        }
        self.1 = vec![]
    }

    fn flash(&mut self, i: usize, j: usize) {
        if !self.1.contains(&(i, j)) {
            self.2 += 1;
            self.1.push((i, j));
            self.0[i][j] = 0;
            if i > 0 {
                self.0[i - 1][j] += 1;
                if self.0[i - 1][j] > 9 {
                    self.flash(i - 1, j)
                }
                if j > 0 {
                    self.0[i - 1][j - 1] += 1;
                    if self.0[i - 1][j - 1] > 9 {
                        self.flash(i - 1, j - 1)
                    }
                }
                if j < 9 {
                    self.0[i - 1][j + 1] += 1;
                    if self.0[i - 1][j + 1] == 9 {
                        self.flash(i - 1, j + 1)
                    }
                }
            }
            if i < 9 {
                self.0[i + 1][j] += 1;
                if self.0[i + 1][j] == 9 {
                    self.flash(i + 1, j)
                }
                if j > 0 {
                    self.0[i + 1][j - 1] += 1;
                    if self.0[i + 1][j - 1] == 9 {
                        self.flash(i + 1, j - 1)
                    }
                }
                if j < 9 {
                    self.0[i + 1][j + 1] += 1;
                    if self.0[i + 1][j + 1] == 9 {
                        self.flash(i + 1, j + 1)
                    }
                }
            }
            if j > 0 {
                self.0[i][j - 1] += 1;
                if self.0[i][j - 1] == 9 {
                    self.flash(i, j - 1)
                }
            }
            if j < 9 {
                self.0[i][j + 1] += 1;
                if self.0[i][j + 1] == 9 {
                    self.flash(i, j + 1)
                }
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
    let number_of_steps = 2;

    let input: Vec<Vec<u32>> = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|number| number.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    let mut octo_grid: OctoGrid = OctoGrid(input, vec![], 0);
    println!("{}", octo_grid);
    let mut i = 0;
    while i < number_of_steps {
        print!("{}[2J", 27 as char);
        octo_grid.step();
        println!("{}", octo_grid);
        i += 1;
    }

    println!("There were {} flashes", octo_grid.2);
}
