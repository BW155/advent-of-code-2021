use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Line {
    x1: i16,
    y1: i16,
    x2: i16,
    y2: i16,
}

impl Line {
    pub fn is_hor_ver(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    pub fn get_line(&self) -> Vec<(i16, i16)> {
        let mut result = Vec::new();
        let length = max((self.x1 - self.x2).abs(), (self.y1 - self.y2).abs());

        let mut x_step = 1;
        let mut y_step = 1;
        let mut cur_x = self.x1;
        let mut cur_y = self.y1;
        if self.x1 > self.x2 {
            x_step = -1;
        } else if self.x1 == self.x2 {
            x_step = 0;
        }
        if self.y1 > self.y2 {
            y_step = -1;
        } else if self.y1 == self.y2 {
            y_step = 0;
        }
        for _ in 0..length + 1{
            result.push((cur_x, cur_y));
            cur_x += x_step;
            cur_y += y_step;
        }
        result
    }
}

fn main() {
    let file = File::open("input").unwrap();
    let lines = io::BufReader::new(file)
        .lines();

    let sublines: Vec<Line> =
        lines
            .filter_map(|f| f.ok())
            .map(|s| s.split(" -> ")
                .map(|s| s.split(",")
                    .filter_map(|f| f.parse::<i16>().ok())
                    .collect::<Vec<i16>>()
                ).collect()
            )
            .map(|xys: Vec<Vec<i16>>| Line { x1: xys[0][0], y1: xys[0][1], x2: xys[1][0], y2: xys[1][1] })
            .collect();

    // Part One
    let hor_ver_lines = sublines.iter().filter(|l| l.is_hor_ver()).map(|l| l.clone()).collect::<Vec<Line>>();
    println!("Count: {}", intersecting_coords(&hor_ver_lines));

    // Part Two
    println!("Count: {}", intersecting_coords(&sublines))
}

fn intersecting_coords(lines: &Vec<Line>) -> usize {
    let mut coords: HashMap<String, i16> = HashMap::new();

    for line in lines {
        line.get_line().iter().for_each(|(x, y)| {
            let stat = coords.entry(format!("{},{}", x, y)).or_insert(0);
            *stat += 1;
        });
    }

    coords.iter().filter(|(_, val)| val > &&1).count()
}
