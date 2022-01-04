use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn solve(path: &Path) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut x = 0;
    let mut y = 0;

    let mut x2: i64 = 0;
    let mut y2: i64 = 0;
    let mut aim: i64 = 0;

    for result in reader.lines() {
        let line = result.unwrap();
        let words: Vec<_> = line.trim_end().split_whitespace().collect();
        let dir = words[0];
        let amount = words[1].parse::<i32>().unwrap();

        match dir {
            "up" => {
                y -= amount;
                aim -= amount as i64;
            }
            "down" => {
                y += amount;
                aim += amount as i64;
            }
            "forward" => {
                x += amount;
                x2 += amount as i64;
                y2 += aim * amount as i64
            }
            _ => {}
        }
    }
    println!("{} {}", x * y, x2 * y2);
}
