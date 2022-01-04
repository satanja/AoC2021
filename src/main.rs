#![allow(non_snake_case)]
#![allow(dead_code)]

use std::path::Path;

mod day01;
mod day02;

fn main() {
    let path = Path::new("./src/day2/input.txt");
    day02::solve(path);
}
