use day_10::process_part_1;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("{}", process_part_1(&input));
}
