use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    let file = File::open("input").unwrap();
    let mut lines = io::BufReader::new(file).lines();

    let positions = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .filter_map(|f| f.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let max = *positions.iter().max().unwrap() as usize;
    let min = *positions.iter().min().unwrap() as usize;

    // Part One

    let best_fuel: u32 = (min..max)
        .map(|i| {
            positions
                .iter()
                .map(|p| (i as i16 - *p as i16).abs() as u32)
                .sum()
        })
        .min()
        .unwrap();
    println!("best fuel: {}", best_fuel);

    // Part Two

    let best_fuel: u32 = (min..max)
        .map(|i| {
            positions
                .iter()
                .map(|p| (i as i32 - *p as i32).abs() as u32)
                .map(|p| (p.pow(2) + p) / 2)
                .sum()
        })
        .min()
        .unwrap();

    println!("Best fuel: {}", best_fuel);
}
