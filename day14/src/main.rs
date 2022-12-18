#![feature(array_windows, array_try_map)]
use std::str::FromStr;
use std::{env, fs};

fn main() {
    let mut args = env::args().skip(1);
    let part = args.next().unwrap();
    let file = args.next().unwrap();
    let content = fs::read_to_string(file).unwrap();

    let mut rock_forms = content
        .lines()
        .map(|line| {
            line.split(" -> ")
                .flat_map(|pos| pos.split_once(','))
                .map(|(x, y)| [x, y])
                .flat_map(|pos| pos.try_map(usize::from_str))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let &[max_x, _] = rock_forms
        .iter()
        .flat_map(|rock| rock)
        .max_by_key(|[x, _]| x)
        .unwrap();

    let &[_, max_y] = rock_forms
        .iter()
        .flat_map(|rock| rock)
        .max_by_key(|[_, y]| y)
        .unwrap();

    println!("max X: {max_x}, max Y: {max_y}");
    if part == "part1" {
        println!("---------- Part 1 -----------");
        let field = vec![vec![false; max_x + 1]; max_y + 1];
        calc_sands(rock_forms, field);
    } else if part == "part2" {
        println!("---------- Part 2 -----------");
        let max_y = max_y + 2;
        let max_x = (max_x + 1).max(500 + (max_y));
        let field = vec![vec![false; max_x + 1]; max_y + 1];
        rock_forms.push(vec![[0, max_y], [max_x, max_y]]);

        calc_sands(rock_forms, field);
    }
}

fn calc_sands(rock_forms: Vec<Vec<[usize; 2]>>, mut field: Vec<Vec<bool>>) {
    for rock_form in &rock_forms {
        for &[rock1, rock2] in rock_form.array_windows::<2>() {
            let x1 = rock1[0].min(rock2[0]);
            let x2 = rock1[0].max(rock2[0]);
            let y1 = rock1[1].min(rock2[1]);
            let y2 = rock1[1].max(rock2[1]);
            for y in y1..=y2 {
                for x in x1..=x2 {
                    field[y][x] = true
                }
            }
        }
    }

    let starting_field = field.clone();
    let max_y = field.len() - 1;
    draw_field(&starting_field, &field);
    let sands = (0..usize::MAX)
        .take_while(|_| {
            let mut x = 500;
            let mut y = 0;
            //draw_field(&starting_field, &field);
            loop {
                let down = field.iter().skip(y).take_while(|line| !line[x]).count();
                if down == 0 {
                    break false;
                }
                y += down - 1;
                if y == max_y {
                    break false;
                }
                if !field[y + 1][x - 1] {
                    y += 1;
                    x -= 1;
                } else if !field[y + 1][x + 1] {
                    y += 1;
                    x += 1;
                } else {
                    field[y][x] = true;
                    break true;
                }
            }
        })
        .count();
    draw_field(&starting_field, &field);
    println!("total sands: {sands}");
}

fn draw_field(start: &Vec<Vec<bool>>, field: &Vec<Vec<bool>>) {
    let height = start.len();
    let width = start[0].len().min(500 + height);
    let start_x = 500 - height;
    for y in 0..height {
        let line = (start_x..width)
            .map(|x| {
                if x == 500 && y == 0 {
                    '+'
                } else if start[y][x] {
                    '#'
                } else if field[y][x] {
                    'o'
                } else {
                    '.'
                }
            })
            .collect::<String>();
        println!("{y:>4}[:::{line}]");
    }
    println!("{:-<124}", '-');
}
