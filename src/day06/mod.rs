use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn solve(path: &Path) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut fishes = [0 as usize; 9];
    for result in reader.lines() {
        let line = result.unwrap();
        let states: Vec<_> = line
            .trim_end()
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        for state in states {
            fishes[state] += 1;
        }
        break;
    }

    for _ in 0..80 {
        fishes.rotate_left(1);
        fishes[6] += fishes[8];
    }

    let p1 = fishes.iter().fold(0, |acc, count| acc + count);

    for _ in 80..256 {
        fishes.rotate_left(1);
        fishes[6] += fishes[8];
    }

    let p2 = fishes.iter().fold(0, |acc, count| acc + count);

    println!("{} {}", p1, p2);
}
