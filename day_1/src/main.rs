use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    let file = File::open("input_example").unwrap();
    let numbers: Vec<u16> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<u16>().unwrap())
        .collect();
    let numbers_len = numbers.len();

    // Part 1
    let mut increase_count: u16 = 0;
    let mut previous: u16 = 0;

    for i in 0..numbers_len {
        if previous < numbers[i] && previous != 0 {
            increase_count += 1;
        }
        previous = numbers[i];
    }
    println!("Increases: {}", increase_count);

    // Part 2
    let mut increase_count: u16 = 0;
    let mut previous: u16 = 0;

    for i in 0..numbers_len {
        if i + 2 < numbers_len {
            let num = numbers[i] + numbers[i + 1] + numbers[i + 2];
            if previous < num && previous != 0 {
                increase_count += 1;
            }
            previous = num;
        }
    }
    println!("Increases: {}", increase_count);
}
