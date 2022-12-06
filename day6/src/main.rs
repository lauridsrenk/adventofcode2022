#![feature(array_windows)]
use std::{env, fs};

fn main() {
    let mut args = env::args().skip(1);
    let part = args.next().unwrap();
    let file = args.next().unwrap();
    let content = fs::read_to_string(file).unwrap();
    let parsed = content.trim();

    if part == "part1" {
        look_for_distinct::<4>(parsed);
    } else if part == "part2" {
        look_for_distinct::<14>(parsed);
    }
}

fn look_for_distinct<const N: usize>(parsed: &str) {
    parsed.lines().for_each(|line| {
        let word = line
            .as_bytes()
            .array_windows::<N>()
            .position(|arr| {
                arr.iter()
                    .enumerate()
                    .all(|(idx, f)| arr[idx + 1..].iter().all(|g| g != f))
            })
            .unwrap()
            + N;

        if line.len() > 35 {
            println!("{}...({}): {}", &line[..35], line.len(), word);
        } else {
            println!("{line:35}: {word}",);
        }
    });
}
