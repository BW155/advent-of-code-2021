use std::fs::File;
use std::io;
use std::io::BufRead;

fn fishing(fish: &Vec<u8>, days: u16) -> u64 {
    let mut timers: Vec<u64> = vec![0; 9];
    fish.iter().for_each(|f| timers[*f as usize] += 1);

    for _ in 0..days {
        let zero = timers[0];

        for age in 1..9 {
            if timers[age] > 0 {
                let add_timer = timers[age];
                timers[age - 1] += add_timer;
                timers[age] -= add_timer;
            }
        }

        timers[6] += zero;
        timers[8] += zero;
        timers[0] -= zero;
    }

    timers.iter().sum()
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
