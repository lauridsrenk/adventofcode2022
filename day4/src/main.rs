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

    if part == "part1" {
        let count = pairs
            .filter(|[a, b, c, d]| ((a - c) * (b - d)) <= 0)
            .count();
        println!("solution to {part}: {count}");
    } else if part == "part2" {
        let count = pairs
            .filter(|[a, b, c, d]| ((a - d) * (b - c)) <= 0)
            .count();
        println!("solution to {part}: {count}");
    }
}
