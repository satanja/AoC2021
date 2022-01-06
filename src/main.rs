use std::path::Path;

// mod day01;
// mod day02;
// mod day03;
// mod day04;
// mod day05;
// mod day06;
mod day07;

fn main() {
    let path = Path::new("./src/day07/input.txt");
    day07::solve(path);
}
