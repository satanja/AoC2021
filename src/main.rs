use std::path::Path;

mod day1;

fn main() {
    let path = Path::new("./src/day1/input.txt");
    day1::solve(path);
}
