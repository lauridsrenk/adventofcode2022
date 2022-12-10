use std::{env, fs};

fn main() {
    let mut args = env::args().skip(1);
    let file = args.next().unwrap();
    let content = fs::read_to_string(file).unwrap();

    let register_vals = content.lines().fold(vec![1], |mut vals, instruction| {
        let last = *vals.last().unwrap();
        vals.push(last);
        if let Some((_, num)) = instruction.split_once(" ") {
            let next: i32 = num.parse().unwrap();
            vals.push(last + next);
        }
        vals
    });

    println!("{0:-<20} Part 1 {0:-<20}", '-');
    let signal_strength: i32 = register_vals
        .iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(mut cycle, &x)| {
            cycle += 1;
            let signal = x * cycle as i32;
            println!("cycle {cycle:>3}, x: {x:>2} => {signal:>4}");
            signal
        })
        .sum();
    println!("total: {signal_strength}");

    println!("\n{0:-<20} Part 2 {0:-<20}", '-');
    let crt_lines = register_vals.chunks(40).map(|line| -> String {
        line.iter()
            .enumerate()
            .map(|(pos, &x)| {
                let diff = pos as i32 - x;
                [' ', '\u{2588}'][(diff == diff.signum()) as usize]
            })
            .collect()
    });
    for line in crt_lines {
        println!("{line}");
    }
}
