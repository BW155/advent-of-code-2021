use std::fs::File;
use std::io;
use std::io::BufRead;

fn convert_bits_num(bits: &Vec<u32>) -> u32 {
    let mut num = 0;
    for bit in bits {
        num = (num << 1) | *bit;
    }
    num
}

const BIT_SIZE: usize = 12;

fn main() {
    let file = File::open("input").unwrap();
    let bitrows = io::BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .split("")
                .filter_map(|s| s.parse::<u32>().ok())
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();

    let bitcols: Vec<Vec<u32>> = (0..BIT_SIZE)
        .map(|i| bitrows.iter().map(|row| row[i]).collect())
        .collect();

    let length: u32 = bitcols[0].len() as u32;

    // Part One

    let commons: Vec<u32> = bitcols
        .iter()
        .map(|bts| {
            if bts.iter().sum::<u32>() > length / 2 {
                1
            } else {
                0
            }
        })
        .collect();
    let uncommon: Vec<u32> = bitcols
        .iter()
        .map(|bts| {
            if bts.iter().sum::<u32>() > length / 2 {
                0
            } else {
                1
            }
        })
        .collect();

    let gamma = convert_bits_num(&commons);
    let epsilon = convert_bits_num(&uncommon);

    println!(
        "gamma: {}, epsilon: {}, multiply: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    // Part Two

    let oxygen_num = find_ratings(&bitrows, true);
    let co2_num = find_ratings(&bitrows, false);

    println!(
        "oxygen: {}, co2: {}, multiply: {}",
        oxygen_num,
        co2_num,
        oxygen_num * co2_num
    );
}

fn find_ratings(data: &Vec<Vec<u32>>, oxygen: bool) -> u32 {
    let mut result: u32 = 0;
    let mut data = data.clone();

    for i in 0..BIT_SIZE {
        if data.len() == 1 {
            result = convert_bits_num(&data[0]);
            break;
        }
        if data.iter().map(|row| row[i]).sum::<u32>() >= (data.len() as u32 / 2) {
            data = data
                .iter()
                .filter(|row| row[i] == oxygen as u32)
                .map(|row| row.clone())
                .collect();
        } else {
            data = data
                .iter()
                .filter(|row| row[i] == !oxygen as u32)
                .map(|row| row.clone())
                .collect();
        }
    }

    result
}
