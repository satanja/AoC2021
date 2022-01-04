#![allow(non_snake_case)]
#![allow(dead_code)]

use std::path::Path;

mod day01;
mod day02;
mod day03;

fn main() {
    let path = Path::new("./src/day03/input.txt");
    day03::solve(path);
}
