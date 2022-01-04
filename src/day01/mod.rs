use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn solve(path: &Path) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    
    let mut previous = usize::MAX;
    let mut p1 = 0;

    let mut a = usize::MAX;
    let mut b = usize::MAX;
    let mut c = usize::MAX;
    let mut p2 = 0;

    for result in reader.lines() {
        let line = result.unwrap();
        
        let next_depth = line.trim_end().parse::<usize>().unwrap();
        if next_depth > previous {
            p1 += 1;
        }
        
        previous = next_depth;
        
        if a == usize::MAX {
            a = next_depth;
            continue;
        }
        if b == usize::MAX {
            b = next_depth;
            continue;
        }
        if c == usize::MAX {
            c = next_depth;
            continue;
        }

        if a < next_depth {
            p2 += 1;
        }

        a = b;
        b = c;
        c = next_depth;

    }

    println!("{} {}", p1, p2);
}   