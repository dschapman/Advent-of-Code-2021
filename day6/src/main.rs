use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap().replace("\n", "");
    let mut school_of_lantern_fish: Vec<u8> = input
        .split(",")
        .map(|item| item.parse::<u8>().unwrap())
        .collect();
    let mut day = 0;
    while day < 256 {
        let mut new_school: Vec<u8> = vec![];
        for fish in school_of_lantern_fish.iter_mut() {
            if *fish == 0 {
                *fish = 6;
                new_school.push(8);
            } else {
                *fish -= 1;
            }
        }
        school_of_lantern_fish.append(&mut new_school);
        day += 1;
    }
    println!("{}", school_of_lantern_fish.len())
}
