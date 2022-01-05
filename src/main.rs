#![allow(non_snake_case)]
#![allow(dead_code)]

use std::path::Path;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    let path = Path::new("./src/day05/input.txt");
    day05::solve(path);
}
