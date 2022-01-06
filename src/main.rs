#![allow(warnings, unused)]

use std::path::Path;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    let path = Path::new("./src/day06/input.txt");
    day06::solve(path);
}
