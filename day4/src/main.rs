#![feature(iter_next_chunk)]
use std::str::FromStr;
use std::{env, fs};

fn main() {
    let mut args = env::args().skip(1);
    let part = args.next().unwrap();
    let file = args.next().unwrap();
    let content = fs::read_to_string(file).unwrap();
    let pairs = content.trim().lines().map(|line| {
        line.split([',', '-'])
            .flat_map(i32::from_str)
            .next_chunk::<4>()
            .unwrap()
    });

    let filter = if part == "part1" {
        |pair: &[i32; 4]| {
            (pair[0] <= pair[2] && pair[1] >= pair[3]) || (pair[0] >= pair[2] && pair[1] <= pair[3])
        }
    } else if part == "part2" {
        |pair: &[i32; 4]| {
            (pair[0] <= pair[2] && pair[1] >= pair[2])
                || (pair[0] <= pair[3] && pair[1] >= pair[3])
                || (pair[0] >= pair[2] && pair[1] <= pair[3])
        }
    } else {
        panic!("part is neither part1 nor part2")
    };

    let count = pairs.filter(filter).count();
    println!("{count}");
}
