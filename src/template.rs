use std::path::Path;
use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn solve(path: &Path) {

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    
    for result in reader.lines() {
        let line = result.unwrap();
    }
}