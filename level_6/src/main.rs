use std::fs::File;
use std::io;
use std::io::BufRead;


fn fishing(fish: &Vec<u8>, days: u16) -> u64 {
    let mut ages: Vec<u64> = vec![0; 9];
    fish.iter().for_each(|f| ages[*f as usize] += 1);

    for _ in 0..days {
        let add = ages[0];

        for age in 1..9 {
            if ages[age] > 0 {
                let add_age = ages[age];
                ages[age - 1] += add_age;
                ages[age] -= add_age;
            }
        }

        ages[6] += add;
        ages[8] += add;
        ages[0] -= add;
    }

    ages.iter().sum()
}

fn main() {
    let file = File::open("input").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let fish = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .filter_map(|f| f.parse::<u8>().ok())
        .collect::<Vec<u8>>();

    // Part One
    let result = fishing(&fish, 80);
    println!("Count: {}", result);

    // Part Two
    let result = fishing(&fish, 256);
    println!("Count: {}", result);
}
