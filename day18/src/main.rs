#![feature(iter_next_chunk)]
use std::{env, fs};

fn main() {
    let mut args = env::args().skip(1);
    let part = args.next().unwrap();
    let file = args.next().unwrap();
    let content = fs::read_to_string(file).unwrap();

    let rocks: Vec<[usize; 3]> = content
        .trim()
        .lines()
        .map(|line| {
            line.split(',')
                .next_chunk()
                .unwrap()
                .map(|word| word.parse().unwrap())
        })
        .collect();

    let max_x = rocks.iter().max_by_key(|[x, ..]| x).unwrap()[0];
    let max_y = rocks.iter().max_by_key(|[_, y, _]| y).unwrap()[1];
    let max_z = rocks.iter().max_by_key(|[.., z]| z).unwrap()[2];
    println!("max x,y,z: {max_x}, {max_y}, {max_z}");

    let mut grid = vec![vec![vec![0; max_x + 3]; max_y + 3]; max_z + 3];
    for [x, y, z] in rocks {
        grid[z + 1][y + 1][x + 1] = 1;
    }

    if part == "part1" {
        let count = count_sides(&grid, 0);
        println!("total sides: {count}");
    } else if part == "part2" {
        fill_external(&mut grid, 2);
        let count = count_sides(&grid, 2);
        println!("total external sides: {count}");
    }
}

fn fill_external(grid: &mut Vec<Vec<Vec<u8>>>, fill_with: u8) {
    println!("filling in...");
    let mut queue = vec![[0i32, 0, 0]];
    while let Some([z, y, x]) = queue.pop() {
        grid[z as usize][y as usize][x as usize] = fill_with;
        let adjacent = [
            [z, y, x - 1],
            [z, y, x + 1],
            [z, y - 1, x],
            [z, y + 1, x],
            [z - 1, y, x],
            [z + 1, y, x],
        ];
        for [z, y, x] in adjacent {
            if z >= 0
                && (z as usize) < grid.len()
                && y >= 0
                && (y as usize) < grid[0].len()
                && x >= 0
                && (x as usize) < grid[0][0].len()
            {
                if grid[z as usize][y as usize][x as usize] == 0 {
                    queue.push([z, y, x]);
                }
            }
        }
    }
}

fn count_sides(grid: &Vec<Vec<Vec<u8>>>, where_side: u8) -> usize {
    println!("counting...");
    let mut count = 0;
    for z in 1..grid.len() - 1 {
        for y in 1..grid[0].len() - 1 {
            for x in 1..grid[0][0].len() - 1 {
                if grid[z][y][x] == 1 {
                    count += [
                        [z, y, x - 1],
                        [z, y, x + 1],
                        [z, y - 1, x],
                        [z, y + 1, x],
                        [z - 1, y, x],
                        [z + 1, y, x],
                    ]
                    .into_iter()
                    .filter(|&[z, y, x]| grid[z][y][x] == where_side)
                    .count();
                }
            }
        }
    }
    count
}
