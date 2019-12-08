use std::fs;

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Something went terrible wrong.");
    let mut vec = get_data(&data);

    let mut cvec = vec.clone();
    vec[1] = 12;
    vec[2] = 2;

    part_one(&mut vec);
    let (noun, verb) = part_two(&mut cvec);
    println!(" Part Two: {}", 100 * noun + verb);
}

pub fn part_one(val: &mut Vec<usize>) {
    intcode_program(val);
    println!("Part One: {}", val[0]);
}

pub fn part_two(val: &mut Vec<usize>) -> (usize, usize) {
    for noun in 0..val.len() - 1 {
        for verb in 0..val.len() - 1 {
            let mut t = val.clone();
            t[1] = noun;
            t[2] = verb;

            intcode_program(&mut t);

            if 19690720 == t[0] {
                return (noun, verb);
            }
        }
    }

    return (0, 0);
}

fn intcode_program(val: &mut Vec<usize>) {
    let mut i: usize = 0;
    loop {
        let i1 = val[i + 1];
        let i2 = val[i + 2];
        let i3 = val[i + 3];
        match val[i] {
            1 => {
                val[i3] = val[i1] + val[i2];
            }
            2 => {
                val[i3] = val[i1] * val[i2];
            }
            99 => break,
            x => panic!("Not a valid opcode"),
        }
        i += 4;
    }
}

pub fn get_data(data: &str) -> Vec<usize> {
    return data
        .split(",")
        .map(|s| s.trim())
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
}
