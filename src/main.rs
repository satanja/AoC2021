#![allow(non_snake_case)]
#![allow(dead_code)]

use std::path::Path;

mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let path = Path::new("./src/day04/input.txt");
    day04::solve(path);
}
