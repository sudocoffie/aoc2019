use std::fs;

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Something went terrible wrong.");
    part_one(&data);
    part_two(&data);
}

pub fn part_one(data: &str) {
}

pub fn part_two(data: &str) {
}