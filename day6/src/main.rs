use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap().replace("\n", "");
    let mut school: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
    input
        .split(",")
        .for_each(|item| school[item.parse::<usize>().unwrap()] += 1);
    let mut day = 0;
    while day < 256 {
        let mut i = 0;
        let mut stage6 = 0;
        let mut stage8 = 0;
        while i < 9 {
            if i == 0 {
                if school[i] > 0 {
                    stage6 += school[i];
                    stage8 += school[i];
                    school[i] -= school[i];
                }
            } else {
                if school[i] > 0 {
                    school[i - 1] += school[i];
                    school[i] -= school[i];
                }
            }
            i += 1;
        }
        school[6] += stage6;
        school[8] += stage8;
        day += 1;
    }
    println!("{:?}", school);
    let sum: usize = school.iter().sum();
    println!("{}", sum)
}
