use std::fs;



fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut grid = vec![vec![0u8; 1000]; 1000];
    let lines = input.lines();
    for line in lines {
        let mut word = line.split_whitespace();
        let mut point1 = word.next().unwrap().split(",");
        word.next();
        let mut point2 = word.next().unwrap().split(",");
        let mut x1 = point1.next().unwrap().parse::<u16>().unwrap();
        let mut y1 = point1.next().unwrap().parse::<u16>().unwrap();
        let x2 = point2.next().unwrap().parse::<u16>().unwrap();
        let y2 = point2.next().unwrap().parse::<u16>().unwrap();
        let x_diff: i32 = i32::from(x1) - i32::from(x2);
        let y_diff:i32 = i32::from(y1) - i32::from(y2);
        if x_diff == 0 {
            if y1 > y2 {
                while y1 >= y2 {
                    grid[y1 as usize][x1 as usize] += 1;
                    println!("Marking {},{}",x1,y1);
                    y1-=1;
                }
            } else if y2 > y1 {
                while y2 >= y1 {
                    grid[y1 as usize][x1 as usize] += 1;
                    println!("Marking {},{}",x1,y1);
                    y1+=1;
                } 
            }
            
        } else if y_diff == 0 {
            if x1 > x2 {
                while x1 >= x2 {
                    grid[y1 as usize][x1 as usize] += 1;
                    println!("Marking {},{}",x1,y1);
                    x1-=1;
                }
            } else if x2 > x1 {
                while x2 >= x1 {
                    grid[y1 as usize ][x1 as usize] += 1;
                    println!("Marking {},{}",x1,y1);
                    x1+=1;
                } 
            }
//For Part 1 I just didn't include the below else statement
        } else {
            if x1 > x2 {
                while x1 >= x2 && y2 >= y1 {
                    grid[y1 as usize][x1 as usize] += 1;
                    println!("Marking {},{}",x2,y1);
                    x1-=1;
                    y1+=1;
                }
                while x1 >= x2 && y1 >= y2 {
                    grid[y1 as usize][x1 as usize] += 1;
                    println!("Marking {},{}",x2,y2);
                    x1-=1;
                    y1-=1; 
                }
            } else if x2 > x1  {
                while x2 >= x1 && y2 >= y1 {
                    grid[y1 as usize][x1 as usize] += 1;
                    println!("Marking {},{}",x1,y1);
                    x1+=1;
                    y1+=1;
                }
                while x2 >= x1 && y1 >= y2 {
                    grid[y1 as usize][x1 as usize] += 1;
                    println!("Marking {},{}",x1,y2);
                    x1+=1;
                    y1-=1;
                }
            }
            
        }
    }
    let mut count = 0;
    for vec in grid {
        for item in vec {
            print!(" {} ",item);
            if item > 1 {
                count +=1
            }
            
        }
        println!();
    }
    println!("{}", count)
}
