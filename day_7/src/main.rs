use std::collections::HashMap;
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
        .filter_map(|f| f.parse::<u16>().ok())
        .collect::<Vec<u16>>();

    let max = *positions.iter().max().unwrap() as usize;
    let min = *positions.iter().min().unwrap() as usize;

    // Part One

    let mut best_i = 0;
    let mut best_fuel = u32::MAX;

    for i in min..max {
        let mut fuel: u32 = 0;
        for pos in &positions {
            fuel += (i as i16 - *pos as i16).abs() as u32;
        }
        if best_fuel > fuel {
            best_i = i;
            best_fuel = fuel;
        }
    }

    println!("Best i: {}, best fuel: {}", best_i, best_fuel);

    // Part Two

    let mut fuel_cost: Vec<u32> = Vec::new();
    let mut cost: u32 = 0;
    for i in 1..max + 2 {
        fuel_cost.push(cost);
        cost += i as u32;
    }

    let mut best_i = 0;
    let mut best_fuel = u32::MAX;

    for i in min..max + 1 {
        let mut fuel: u32 = 0;
        for pos in &positions {
            fuel += fuel_cost[(i as i16 - *pos as i16).abs() as usize];
        }
        if best_fuel > fuel {
            best_i = i;
            best_fuel = fuel;
        }
    }

    println!("Best i: {}, best fuel: {}", best_i, best_fuel);
}
