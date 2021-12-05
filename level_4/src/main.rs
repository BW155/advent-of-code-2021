use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Debug, Clone, PartialEq)]
struct Board {
    cols: Vec<Vec<u8>>,
    rows: Vec<Vec<u8>>,
}

impl Board {
    pub fn check_called_numbers(self, numbers: &Vec<u8>) -> bool {
        if self
            .cols
            .iter()
            .any(|col| col.iter().all(|c| numbers.contains(c)))
        {
            return true;
        }
        if self
            .rows
            .iter()
            .any(|row| row.iter().all(|c| numbers.contains(c)))
        {
            return true;
        }
        false
    }

    pub fn get_unchecked_numbers(&self, numbers: &Vec<u8>) -> Vec<u16> {
        let mut result = Vec::new();
        self.rows.iter().for_each(|row| {
            result.extend(
                row.iter()
                    .filter(|num| !numbers.contains(num))
                    .map(|num| *num as u16),
            )
        });
        result
    }
}

fn main() {
    let file = File::open("input").unwrap();
    let mut lines = io::BufReader::new(file).lines();

    let mut numbers: Vec<u8> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .filter_map(|s| s.parse::<u8>().ok())
        .collect();
    let mut boards = Vec::new();

    let mut peek = lines.peekable();
    while peek.peek().is_some() {
        let n = peek.next().unwrap().unwrap();
        if n == "" {
            let mut rows: Vec<Vec<u8>> = Vec::new();
            for _ in 0..5 {
                let row = peek.next().unwrap().unwrap();
                rows.push(
                    row.split(" ")
                        .filter_map(|s| s.parse::<u8>().ok())
                        .collect(),
                )
            }
            let cols: Vec<Vec<u8>> = (0..5)
                .map(|i| rows.iter().map(|row| row[i]).collect())
                .collect();
            boards.push(Board { rows, cols });
        }
    }

    println!("numbers: {}, boards: {}", numbers.len(), boards.len());

    // Part One

    {
        let mut numbers = numbers.clone();
        let mut take: usize = 5;
        let mut numbers_called = Vec::new();
        'partone: loop {
            for _ in 0..take {
                numbers_called.extend(numbers.drain(0..1));
                for b in &boards {
                    if b.clone().check_called_numbers(&numbers_called) {
                        let last_number = *numbers_called.last().unwrap() as u16;
                        let sum: u16 = b
                            .clone()
                            .get_unchecked_numbers(&numbers_called)
                            .iter()
                            .sum();
                        println!("WINNER");
                        println!("Winning number: {}", last_number);
                        println!("Final score: {}", sum * last_number);
                        break 'partone;
                    }
                }
            }
            take += 1;
        }
    }

    // Part two

    let mut remaining_boards = boards.clone();
    let mut take: usize = 5;
    let mut numbers_called = Vec::new();
    'parttwo: loop {
        for _ in 0..take {
            numbers_called.extend(numbers.drain(0..1));
            for b in remaining_boards.clone().iter() {
                if b.clone().check_called_numbers(&numbers_called) {
                    if remaining_boards.len() == 1 {
                        let last_number = *numbers_called.last().unwrap() as u16;
                        let sum: u16 = b
                            .clone()
                            .get_unchecked_numbers(&numbers_called)
                            .iter()
                            .sum();
                        println!("LAST WINNER");
                        println!("LAST Winning number: {}", last_number);
                        println!("LAST Final score: {}", sum * last_number);
                        break 'parttwo;
                    }
                    let index = remaining_boards.iter().position(|x| x == b).unwrap();
                    remaining_boards.remove(index);
                }
            }
        }
        take += 1;
    }
}
