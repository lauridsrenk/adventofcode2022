#![feature(array_zip)]
use std::collections::HashSet;
use std::{env, fs};

fn main() {
    let mut args = env::args().skip(1);
    let file = args.next().unwrap();
    let content = fs::read_to_string(file).unwrap();
    let show = args.next().map_or(false, |arg| arg == "--show-field");

    let parsed: Vec<_> = content
        .lines()
        .flat_map(|line| {
            let (direction, amount) = line.split_once(" ")?;
            let amount = amount.parse().ok()?;
            let delta = match direction {
                "L" => [-1, 0],
                "R" => [1, 0],
                "U" => [0, -1],
                "D" => [0, 1],
                _ => return None,
            };
            Some((delta, amount))
        })
        .collect();

    println!("----- Part 1 ------");
    calc_rope::<2>(&parsed, show);
    println!("----- Part 2 -----");
    calc_rope::<10>(&parsed, show);
}

fn calc_rope<const N: usize>(instructions: &[([i32; 2], usize)], show_field: bool) {
    let field: HashSet<_> = instructions
        .into_iter()
        .flat_map(|&(delta, amount)| [delta].into_iter().cycle().take(amount))
        .scan([[0i32, 0]; N], |rope, delta| {
            rope[0] = rope[0].zip(delta).map(|(h, d)| h + d);
            for i in 1..N {
                let delta = rope[i - 1].zip(rope[i]).map(|(h, t)| (h - t));
                if delta[0].abs() > 1 || delta[1].abs() > 1 {
                    rope[i] = rope[i].zip(delta).map(|(t, d)| t + d.signum());
                }
            }
            Some(rope[N - 1])
        })
        .collect();

    if show_field {
        for y in -60..10 {
            for x in -70..20 {
                if x == 0 && y == 0 {
                    print!("S ");
                } else if field.contains(&[x, y]) {
                    print!("# ");
                } else {
                    print!(". ");
                }
            }
            println!("");
        }
    }

    let count = field.into_iter().count();
    println!("touched: {count}");
}
