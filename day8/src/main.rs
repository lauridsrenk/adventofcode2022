use std::{env, fs};

fn main() {
    let file = env::args().skip(1).next().unwrap();
    let content = fs::read_to_string(file).unwrap();

    let parsed: Vec<_> = content
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect();
    let h = parsed.len();
    let w = parsed[0].len();

    println!("----- Part 1 -----");
    let visible: usize = parsed
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            println!("");
            line.iter().enumerate().zip([y].into_iter().cycle())
        })
        .filter(|&((x, &ht), y)| {
            let tp = (0..y).all(|y| parsed[y][x] < ht);
            let lt = (0..x).all(|x| parsed[y][x] < ht);
            let bm = (y + 1..h).all(|y| parsed[y][x] < ht);
            let rt = (x + 1..w).all(|x| parsed[y][x] < ht);
            let vs = tp || lt || bm || rt;
            print!("{} ", if vs { ht.to_string() } else { '.'.into() });
            vs
        })
        .count();
    println!("\nvisible: {visible}");

    println!("\n----- Part 2 -----\n");
    let top_score = parsed
        .iter()
        .enumerate()
        .flat_map(|(y, line)| line.iter().enumerate().zip([y].into_iter().cycle()))
        .map(|((x, &ht), y)| {
            let tp = y - (0..y).rposition(|y| parsed[y][x] >= ht).unwrap_or(0);
            let lt = x - (0..x).rposition(|x| parsed[y][x] >= ht).unwrap_or(0);
            let bm = (y + 1..h)
                .position(|y| parsed[y][x] >= ht)
                .map_or(h - y - 1, |p| p + 1);
            let rt = (x + 1..w)
                .position(|x| parsed[y][x] >= ht)
                .map_or(w - x - 1, |p| p + 1);
            tp * lt * bm * rt
        })
        .max()
        .unwrap();
    println!("top score: {top_score}");
}
