use std::fs;

#[allow(dead_code)]
fn part1() {
    //this will read in the file as bytes and convert them into decimal which is fine
    let mut gamma = 0;
    let mut epsilon = 0;
    let input = fs::read_to_string("input.txt").unwrap();
    let bytes: Vec<u16> = input
        .lines()
        .map(|line| u16::from_str_radix(line, 2).unwrap())
        .collect::<Vec<u16>>();
    // We want to know whether 1 or 0 is the most common first digit in our set of numbers
    // To determine whether a number starts with 0 or 1 we just need to find out if its divisible by 2 (10):
    // http://www.pages.drexel.edu/~dpl49/binary.html#:~:text=The%20first%20digit%20of%20a,or%20mixed%20up%20the%20two.
    // then because its a u16 we can just loop through 16 times right?

    //Need to use a reference and then use * to access the data
    //
    let mut i = 16;
    while i > 0 {
        let mut count0 = 0;
        let mut count1 = 0;
        for bit in &bytes {
            let number = *bit >> i - 1;
            if number % 2 == 0 {
                count0 += 1;
            } else {
                count1 += 1;
            }
        }
        if count0 > count1 {
            gamma += 0;
            if gamma != 0 {
                epsilon += u16::pow(2, i - 1);
            }
        } else {
            gamma += u16::pow(2, i - 1);
            epsilon += 0;
        }
        println!("At i: {}, gamma is {}, epsilon is {}", i, gamma, epsilon);
        i -= 1;
    }

    let result: u32 = u32::from(gamma) * u32::from(epsilon);
    println!(
        "gamma: {:b}, epsilon: {:b}, result: {}",
        gamma, epsilon, result
    )
}

fn part2() {
    //this will read in the file as bytes and convert them into decimal which is fine
    let input = fs::read_to_string("input.txt").unwrap();
    let bytes: Vec<u16> = input
        .lines()
        .map(|line| u16::from_str_radix(line, 2).unwrap())
        .collect::<Vec<u16>>();
    let mut oxygen = bytes.clone();
    let mut oxygen_result: Vec<u16> = vec![];
    let mut co2 = bytes.clone();
    let mut co2_result: Vec<u16> = vec![];
    while oxygen.len() > 1 {
        let mut i = 16;
        while i > 0 {
            let mut count0 = 0;
            let mut count1 = 0;
            for bit in &oxygen {
                let number = *bit >> i - 1;
                if number % 2 == 0 {
                    count0 += 1;
                } else {
                    count1 += 1;
                }
            }
            if count0 > count1 {
                while oxygen.is_empty() != true {
                    let potential = oxygen.pop().unwrap();
                    match (potential >> i - 1) % 2 {
                        //if 0 store the potential correct value
                        0 => oxygen_result.push(potential),
                        1 => continue,
                        _ => break,
                    }
                }
            } else {
                while oxygen.is_empty() != true {
                    let potential = oxygen.pop().unwrap();
                    match (potential >> i - 1) % 2 {
                        //if 0 store the potential correct value
                        0 => continue,
                        1 => oxygen_result.push(potential),
                        _ => break,
                    }
                }
            }
            oxygen = oxygen_result;
            oxygen_result = vec![];
            i -= 1;
        }
    }
    while co2.len() > 1 {
        //because the leading 0 throw it off I have to hard set this based on the dataset. The input.txt has 12 digit long binary so that's what it is here.
        let mut i = 12;
        while i > 0 {
            let mut count0 = 0;
            let mut count1 = 0;
            for bit in &co2 {
                let number = *bit >> i - 1;
                println!("{}", number);
                if number % 2 == 0 {
                    count0 += 1;
                } else {
                    count1 += 1;
                }
            }
            if count0 <= count1 {
                while co2.is_empty() != true {
                    let potential = co2.pop().unwrap();
                    match (potential >> i - 1) % 2 {
                        //if 0 store the potential correct value
                        0 => co2_result.push(potential),
                        1 => continue,
                        _ => break,
                    }
                }
            } else {
                while co2.is_empty() != true {
                    let potential = co2.pop().unwrap();
                    match (potential >> i - 1) % 2 {
                        //if 1 store the potential correct value
                        0 => continue,
                        1 => co2_result.push(potential),
                        _ => break,
                    }
                }
            }

            co2 = co2_result;
            co2_result = vec![];
            if co2.len() == 1 {
                break;
            }
            i -= 1;
        }
    }

    let oxygen_rating = oxygen.pop().unwrap();
    let co2_rating = co2.pop().unwrap();
    let result: u32 = u32::from(oxygen_rating) * u32::from(co2_rating);
    println!(
        "o2: {:b}, co2: {:b}, result: {}",
        oxygen_rating, co2_rating, result
    )
}

fn main() {
    part2();
}
