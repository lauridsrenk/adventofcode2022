use std::fs;

fn main() {
    let mut args = std::env::args().skip(1);
    let part = args.next().expect("expected part1 or part2");
    let file = args.next().expect("expected file path");
    let mut sums: Vec<i32> = fs::read_to_string(file)
        .unwrap()
        .split("\r\n\r\n")
        .map(|block| {
            block
                .trim()
                .lines()
                .map(|line| line.parse::<i32>().unwrap())
                .sum()
        })
        .collect();
    if part == "part1" {
        let res = sums.iter().max();
        println!("{res:?}");
    } else if part == "part2" {
        sums.sort();
        sums.reverse();
        if let [first, second, third, ..] = &sums[..] {
            println!(
                "{first}, {second}, {third}, answer to part2: {}",
                first + second + third
            );
        } else {
            println!("wrong shape: {sums:?}");
        }
    }
}
