#![feature(iter_array_chunks)]
use std::{env, fs};

fn main() {
    let mut args = env::args().skip(1);
    let part = args.next().unwrap();
    let file = args.next().unwrap();
    let content = fs::read_to_string(file).unwrap();

    if part == "part1" {
        let prios = content
            .trim()
            .lines()
            .map(|line| {
                let (left, right) = line.split_at(line.len() / 2);
                (65u8..=122)
                    .find(|&idx| {
                        left.chars().find(|&l| l == idx as char).is_some()
                            && right.chars().find(|&r| r == idx as char).is_some()
                    })
                    .unwrap() as i32
            })
            .map(char_prio);
        let sum = prios.sum::<i32>();
        println!("sum is {sum}");
    }

    if part == "part2" {
        let prios = content
            .trim()
            .lines()
            .array_chunks::<3>()
            .map(|[first, second, third]| {
                (65u8..=122)
                    .find(|&idx| {
                        first.chars().find(|&l| l == idx as char).is_some()
                            && second.chars().find(|&r| r == idx as char).is_some()
                            && third.chars().find(|&r| r == idx as char).is_some()
                    })
                    .unwrap() as i32
            })
            .map(char_prio);
        let sum = prios.sum::<i32>();
        println!("sum is {sum}");
    }
}
fn char_prio(alpha: i32) -> i32 {
    if alpha < 97 {
        alpha as i32 - 38
    } else {
        alpha as i32 - 96
    }
}
