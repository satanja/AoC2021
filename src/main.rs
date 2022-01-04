use std::path::Path;

mod day1;
mod day2;

fn main() {
    let path = Path::new("./src/day2/input.txt");
    day2::solve(path);
}
