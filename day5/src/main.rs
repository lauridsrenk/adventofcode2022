#![feature(iter_array_chunks)]
use std::str::FromStr;
use std::{env, fs};

fn main() {
    let mut args = env::args().skip(1);
    let part = args.next().unwrap();
    let file = args.next().unwrap();
    let content = fs::read_to_string(file).unwrap();

    let (stack, instructions) = content.split_once("\r\n\r\n").unwrap();
    let raw_stack: Vec<_> = stack
        .lines()
        .map(|line| line.chars().skip(1).step_by(4).collect::<Vec<_>>())
        .rev()
        .skip(1)
        .collect();
    let mut stack = vec![Vec::new(); raw_stack[0].len()];
    for row in raw_stack {
        for (idx, c) in row
            .into_iter()
            .enumerate()
            .filter(|(_, c)| c.is_alphabetic())
        {
            stack[idx].push(c);
        }
    }

    let instructions = instructions
        .trim()
        .split_whitespace()
        .skip(1)
        .step_by(2)
        .flat_map(usize::from_str)
        .array_chunks::<3>();

    println!("{stack:?}");
    println!("-------------------------------------------");

    if part == "part1" {
        for [count, from, to] in instructions {
            for _ in 0..count {
                let item = stack[from - 1].pop().unwrap();
                stack[to - 1].push(item);
            }
        }
    } else if part == "part2" {
        for [count, from, to] in instructions {
            let len = stack[from - 1].len() - count;
            let mut items = stack[from - 1].split_off(len);
            stack[to - 1].append(&mut items);
        }
    }
    println!("{stack:?}");
    println!("-------------------------------------------");
    let s: String = stack.into_iter().map(|col| *col.last().unwrap()).collect();
    println!("{s}");
}
