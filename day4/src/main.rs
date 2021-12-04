use ansi_term::Color::Green;
use std::fmt;
use std::fs;

#[derive(Debug)]
struct BingoSquare(u8, bool);

#[derive(Debug)]
struct BingoBoard(Vec<BingoSquare>);
impl BingoBoard {
    fn mark(&mut self, num: u8) {
        for square in self.0.iter_mut() {
            if square.0 == num {
                square.1 = true;
            }
        }
    }
    fn check(&self) -> bool {
        if self.0[0].1 && self.0[1].1 && self.0[2].1 && self.0[3].1 && self.0[4].1 {
            return true;
        } else if self.0[5].1 && self.0[6].1 && self.0[7].1 && self.0[8].1 && self.0[9].1 {
            return true;
        } else if self.0[10].1 && self.0[11].1 && self.0[12].1 && self.0[13].1 && self.0[14].1 {
            return true;
        } else if self.0[15].1 && self.0[16].1 && self.0[17].1 && self.0[18].1 && self.0[19].1 {
            return true;
        } else if self.0[20].1 && self.0[21].1 && self.0[22].1 && self.0[23].1 && self.0[24].1 {
            return true;
        } else if self.0[0].1 && self.0[5].1 && self.0[10].1 && self.0[15].1 && self.0[20].1 {
            return true;
        } else if self.0[1].1 && self.0[6].1 && self.0[11].1 && self.0[16].1 && self.0[21].1 {
            return true;
        } else if self.0[2].1 && self.0[7].1 && self.0[12].1 && self.0[17].1 && self.0[22].1 {
            return true;
        } else if self.0[3].1 && self.0[8].1 && self.0[13].1 && self.0[18].1 && self.0[23].1 {
            return true;
        } else if self.0[4].1 && self.0[9].1 && self.0[14].1 && self.0[19].1 && self.0[24].1 {
            return true;
        } else {
            return false;
        }
    }
}

impl fmt::Display for BingoSquare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.1 {
            write!(f, "{}", Green.normal().paint(self.0.to_string()))
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bingo Board:\n")?;
        let mut i = 0;
        for x in &self.0 {
            if i > 4 {
                i = 0;
                write!(f, "\n {} ", x)?;
                i += 1;
            } else {
                write!(f, " {} ", x)?;
                i += 1;
            }
        }
        Ok(())
    }
}
#[allow(dead_code)]
fn part1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines().peekable();
    let numbers: Vec<u8> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.to_string().parse::<u8>().unwrap())
        .collect();

    let mut boards: Vec<BingoBoard> = vec![];
    //parse file
    while lines.peek() != None {
        let mut board: String = "".to_string();
        //Assume we can read 6 lines the first one will be blank
        lines.next();
        board.push_str(lines.next().unwrap());
        board.push_str(" ");
        board.push_str(lines.next().unwrap());
        board.push_str(" ");
        board.push_str(lines.next().unwrap());
        board.push_str(" ");
        board.push_str(lines.next().unwrap());
        board.push_str(" ");
        board.push_str(lines.next().unwrap());
        boards.push(BingoBoard(
            board
                .split_whitespace()
                .map(|square| BingoSquare(square.parse::<u8>().unwrap(), false))
                .collect(),
        ));
    }
    let mut result: (usize, u8, bool) = (0, 0, false);
    'outer: for num in numbers {
        let mut i = 0;
        while i < boards.len() {
            boards[i].mark(num);
            if boards[i].check() {
                result = (i, num, true);
                break 'outer;
            }
            i += 1;
        }
    }

    /* for board in boards {
        println!("{}", board)
    } */
    if result.2 == true {
        println!(
            "Board {} wins at number: {}:\n{}",
            result.0 + 1,
            result.1,
            boards[result.0]
        );
        let mut score: u16 = 0;
        for square in boards[result.0].0.iter() {
            if !square.1 {
                score += u16::from(square.0);
            }
        }
        score *= u16::from(result.1);
        println!("Score: {}", score);
    }
}

fn part2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines().peekable();
    let numbers: Vec<u8> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.to_string().parse::<u8>().unwrap())
        .collect();

    let mut boards: Vec<BingoBoard> = vec![];
    //parse file
    while lines.peek() != None {
        let mut board: String = "".to_string();
        //Assume we can read 6 lines the first one will be blank
        lines.next();
        board.push_str(lines.next().unwrap());
        board.push_str(" ");
        board.push_str(lines.next().unwrap());
        board.push_str(" ");
        board.push_str(lines.next().unwrap());
        board.push_str(" ");
        board.push_str(lines.next().unwrap());
        board.push_str(" ");
        board.push_str(lines.next().unwrap());
        boards.push(BingoBoard(
            board
                .split_whitespace()
                .map(|square| BingoSquare(square.parse::<u8>().unwrap(), false))
                .collect(),
        ));
    }
    let mut result: (usize, u8, bool) = (0, 0, false);
    let mut winning_boards = vec![];
    'outer: for num in numbers {
        let mut i = 0;
        
        while i < boards.len() {
            if !winning_boards.contains(&i) {
                boards[i].mark(num);
                println!("Marking board: {}", i + 1);
            }

            if boards[i].check() && !winning_boards.contains(&i) &&winning_boards.len() < boards.len() -1 {
                winning_boards.push(i);
                println!("Board {} has won.\n{:?}", i+1,winning_boards);
                
            }
            else if boards[i].check()
                && winning_boards.len() == boards.len() -1
                && !winning_boards.contains(&i)
            {
                result = (i, num, true);
                break 'outer;
            } else if boards[i].check() && winning_boards.contains(&i) {
                println!("Board {} has already won.", i+1)
            }
            i += 1;
        }
    }

    if result.2 == true {
        println!(
            "Board {} wins at number: {}:\n{}",
            result.0 + 1,
            result.1,
            boards[result.0]
        );
        let mut score: u16 = 0;
        for square in boards[result.0].0.iter() {
            if !square.1 {
                score += u16::from(square.0);
            }
        }
        println!("Sum of unmarked: {}", score);
        score *= u16::from(result.1);
        println!("Score: {}", score);
    }
}

fn main() {
    part1();
}
