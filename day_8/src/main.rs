use std::fs::File;
use std::io;
use std::io::BufRead;

fn compare(s1: &str, s2: &str) -> bool {
    let mut v1 = s1.chars().collect::<Vec<char>>();
    v1.sort_by(|a, b| a.cmp(b));
    let mut v2 = s2.chars().collect::<Vec<char>>();
    v2.sort_by(|a, b| a.cmp(b));
    v1 == v2
}

fn filter_unique_add(s1: &str, s2: &str) -> String {
    let mut vec = (s1.to_string() + s2).chars().collect::<Vec<char>>();
    vec.dedup();
    vec.iter().collect()
}

fn get_patterns(patterns: &Vec<String>) -> Vec<String> {
    let mut result = vec![String::new(); 10];
    let mut pattern_rest = Vec::new();

    for pat in patterns {
        match pat.len() {
            2 => result[1] = pat.to_string(),
            4 => result[4] = pat.to_string(),
            3 => result[7] = pat.to_string(),
            7 => result[8] = pat.to_string(),
            _ => pattern_rest.push(pat.to_string()),
        }
    }
    result[9] = pattern_rest
        .iter()
        .filter(|p| p.len() == 6 && result[4].split("").all(|rs| p.contains(rs)))
        .next()
        .unwrap_or(&String::new())
        .to_string();
    pattern_rest.retain(|p| p != &result[9]);
    result[0] = pattern_rest
        .iter()
        .filter(|p| p.len() == 6 && result[1].split("").all(|rs| p.contains(rs)))
        .next()
        .unwrap_or(&String::new())
        .to_string();
    pattern_rest.retain(|p| p != &result[0]);
    result[6] = pattern_rest
        .iter()
        .filter(|p| p.len() == 6)
        .next()
        .unwrap_or(&String::new())
        .to_string();
    pattern_rest.retain(|p| p != &result[6]);

    result[3] = pattern_rest
        .iter()
        .filter(|p| p.len() == 5 && result[1].split("").all(|rs| p.contains(rs)))
        .next()
        .unwrap_or(&String::new())
        .to_string();
    pattern_rest.retain(|p| p != &result[3]);
    result[5] = pattern_rest
        .iter()
        .filter(|p| p.len() == 5 && p.split("").all(|rs| result[9].contains(rs)))
        .next()
        .unwrap_or(&String::new())
        .to_string();
    pattern_rest.retain(|p| p != &result[5]);
    result[2] = pattern_rest
        .iter()
        .filter(|p| p.len() == 5)
        .next()
        .unwrap_or(&String::new())
        .to_string();
    result
}

fn main() {
    let file = File::open("input").unwrap();
    let mut lines = io::BufReader::new(file).lines();

    let lines: Vec<(Vec<String>, Vec<String>)> = lines
        .filter_map(|f| f.ok())
        .map(|s| {
            s.split(" | ")
                .map(|s| String::from(s))
                .collect::<Vec<String>>()
        })
        .map(|io| {
            (
                io[0].split(" ").map(|s| String::from(s)).collect(),
                io[1].split(" ").map(|s| String::from(s)).collect(),
            )
        })
        .collect();

    // Part One
    let mut count = 0;

    for (input, outputs) in &lines {
        for output in outputs {
            match output.len() {
                2 => count += 1,
                3 => count += 1,
                4 => count += 1,
                7 => count += 1,
                _ => {}
            }
        }
    }

    println!("Count: {}", count);

    // Part Two
    let mut count = 0;

    for (input, line) in &lines {
        let sets = get_patterns(input);
        let mut line_num = String::new();
        for output in line {
            for s in 0..sets.len() {
                if compare(&output, &sets[s]) {
                    line_num += &format!("{}", s);
                }
            }
        }
        count += line_num.parse::<u32>().unwrap_or(0);
    }

    println!("Count: {}", count);
}
