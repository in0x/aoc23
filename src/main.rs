mod common;
mod day1;

fn main() {
    let cwd = std::env::current_dir().unwrap();
    day1::run(&cwd.join("input/day1.txt"));
}
