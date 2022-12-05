#![feature(iter_next_chunk)]
use std::str::FromStr;
use std::{env, fs};

fn main() {
    let mut args = env::args().skip(1);
    let part = args.next().unwrap();
    let file = args.next().unwrap();
    let content = fs::read_to_string(file).unwrap();
    let pairs = content.trim().lines().map(|line| {
        let tasks = line.split_once(",").unwrap();
        [tasks.0, tasks.1].map(|task| {
            task.split("-")
                .flat_map(i32::from_str)
                .next_chunk::<2>()
                .unwrap()
        })
    });

    if part == "part1" {
        let overlap = pairs
            .filter(|pair| {
                (pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][1])
                    || (pair[0][0] >= pair[1][0] && pair[0][1] <= pair[1][1])
            })
            .inspect(|pair| println!("{pair:?}"));
        let answ = overlap.count();
        println!("answer to part 1: {answ}");
    } else if part == "part2" {
        let overlap = pairs
            .filter(|pair| {
                (pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][0])
                    || (pair[0][0] <= pair[1][1] && pair[0][1] >= pair[1][1])
                    || (pair[0][0] >= pair[1][0] && pair[0][1] <= pair[1][1])
            })
            .inspect(|pair| println!("{pair:?}"));
        let answ = overlap.count();
        println!("answer to part 2: {answ}");
    }
}
