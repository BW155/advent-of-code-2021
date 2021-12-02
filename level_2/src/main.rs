use std::fs::File;
use std::io;
use std::io::BufRead;


struct Movement {
    pub direction: String,
    pub amount: i32,
}

fn main() {
    let file = File::open("input").unwrap();
    let movements: Vec<Movement> = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().split(" ")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>()
        )
        .map(|split| Movement {direction: split[0].clone(), amount: split[1].parse().unwrap()})
        .collect();


    // Part One
    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;

    for m in &movements {
        match m.direction.as_ref() {
            "forward" => horizontal += m.amount,
            "down" => depth += m.amount,
            "up" => depth -= m.amount,
            _ => panic!("ERROR, movement unknown")
        }
    }

    println!("Depth: {}, Horizontal: {}, Multiplied: {}", depth, horizontal, depth * horizontal);

    // Part Two
    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;
    let mut aim: i32 = 0;

    for m in movements {
        match m.direction.as_ref() {
            "forward" => {
                horizontal += m.amount;
                depth += m.amount * aim;
            },
            "down" => aim += m.amount,
            "up" => aim -= m.amount,
            _ => panic!("ERROR, movement unknown")
        }
    }

    println!("Depth: {}, Horizontal: {}, Multiplied: {}", depth, horizontal, depth * horizontal);
}
