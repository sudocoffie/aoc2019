use std::fs;

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Something went terrible wrong.");
    part_one(&data);
    part_two(&data);
}

pub fn fuel(mass: i32) -> i32 {
    return (mass / 3) - 2 as i32;
}

pub fn part_one(data: &str) {
    let mut req: i32 = 0;

    for s in data.split("\n") {
        let mass: i32 = s.trim().parse().unwrap();
        req = req + fuel(mass);
    }

    println!("Part One: {:?}", req);
}

pub fn part_two(data: &str) {
    let mut req: i32 = 0;

    for s in data.split("\n") {
        let mass: i32 = s.trim().parse().unwrap();
        let res = calc_fuel(mass, 0);
        req += res.1;
    }

    println!("Part Two: {:?}", req);
}

pub fn calc_fuel(mass: i32, mut req: i32) -> (i32, i32) {
    let res = fuel(mass);

    if res <= 0 {
        return (res, req);
    }

    req += res;
    return calc_fuel(res, req);
}
