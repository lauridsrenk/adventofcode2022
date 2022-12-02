use std::{env, fs};

fn main() {
    let mut args = env::args().skip(1);
    let part = args.next().unwrap();
    let file = args.next().unwrap();

    let content = fs::read_to_string(file).unwrap();
    let parsed = content.trim().lines().map(|line| {
        let v = line.split(" ").collect::<Vec<&str>>();
        <[&str; 2]>::try_from(v).unwrap()
    });

    if part == "part1" {
        let win = parsed
            .map(|round| match round {
                ["A", "X"] => 3 + 1,
                ["B", "X"] => 0 + 1,
                ["C", "X"] => 6 + 1,
                ["A", "Y"] => 6 + 2,
                ["B", "Y"] => 3 + 2,
                ["C", "Y"] => 0 + 2,
                ["A", "Z"] => 0 + 3,
                ["B", "Z"] => 6 + 3,
                ["C", "Z"] => 3 + 3,
                _ => panic!("{round:?} is invalid round"),
            })
            .sum::<i32>();
        println!("total score for part1: {}", win);
    } else if part == "part2" {
        let win = parsed
            .map(|round| match round {
                ["A", "X"] => 0 + 3,
                ["B", "X"] => 0 + 1,
                ["C", "X"] => 0 + 2,
                ["A", "Y"] => 3 + 1,
                ["B", "Y"] => 3 + 2,
                ["C", "Y"] => 3 + 3,
                ["A", "Z"] => 6 + 2,
                ["B", "Z"] => 6 + 3,
                ["C", "Z"] => 6 + 1,
                _ => panic!("{round:?} is invalid round"),
            })
            .sum::<i32>();
        println!("total score for part2: {}", win);
    }
}
