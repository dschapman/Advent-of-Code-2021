use std::fs;

fn main() {
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
