use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut grid = vec![vec![0u8; 10]; 10];
    let lines = input.lines();
    for line in lines {
        let mut word = line.split_whitespace();
        let mut point1 = word.next().unwrap().split(",");
        word.next();
        let mut point2 = word.next().unwrap().split(",");
        let mut x1 = point1.next().unwrap().parse::<u8>().unwrap();
        let mut y1 = point1.next().unwrap().parse::<u8>().unwrap();
        let mut x2 = point2.next().unwrap().parse::<u8>().unwrap();
        let mut y2 = point2.next().unwrap().parse::<u8>().unwrap();
        let x_diff: i16 = i16::from(x1) - i16::from(x2);
        let y_diff:i16 = i16::from(y1) - i16::from(y2);
        if x_diff == 0 {
            if y1 > y2 {
                while y1 > y2 {
                    grid[y2 as usize][x1 as usize] += 1;
                    y2+=1;
                }
            } else if y2 > y1 {
                while y2 > y1 {
                    grid[y1 as usize][x1 as usize] += 1;
                    y1+=1;
                } 
            }
            
        } else if y_diff == 0 {
            if x1 > x2 {
                while x1 > x2 {
                    grid[y1 as usize][x2 as usize] += 1;
                    x2+=1;
                }
            } else if x2 > x1 {
                while x2 > x1 {
                    grid[y1 as usize ][x1 as usize] += 1;
                    x1+=1;
                } 
            }
        }
        println!("{},{}", x1, y1);
        println!("{},{}", x2, y2);
    }
    let mut count = 0;
    for vec in grid {
        for item in vec {
            if item > 1 {
                count +=1
            }
        }
    }
    
    println!("{}", count)
}
