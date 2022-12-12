use std::collections::HashSet;
use std::collections::VecDeque;
use std::{env, fs};

fn main() {
    let mut args = env::args().skip(1);
    let file = args.next().unwrap();
    let content = fs::read_to_string(file).unwrap();
    let draw = args.next().map_or(false, |arg| arg == "--draw-field");

    println!("----- Part 1 -----");
    let start = content
        .lines()
        .enumerate()
        .find_map(|(y, line)| Some((y, line.find('S')?)))
        .map(|(y, x)| Cell {
            x,
            y,
            value: 'a',
            g: 0,
            h: 0,
        })
        .unwrap();
    let distance = search_path(&content, &start, draw);
    println!("minimum distance from S: {distance:?}");

    println!("----- Part 2 ------");
    let positions: Vec<_> = content
        .lines()
        .enumerate()
        .filter_map(|(y, line)| Some((y, line.find(['S', 'a'])?)))
        .map(|(y, x)| Cell {
            x,
            y,
            value: 'a',
            g: 0,
            h: 0,
        })
        .collect();
    println!("starting positions: {}", positions.len());
    let distance = positions
        .into_iter()
        .filter_map(|start| search_path(&content, &start, false))
        .min()
        .unwrap();
    println!("total minimum distance: {distance:?}");
}

#[derive(Debug, Clone)]
struct Cell {
    x: usize,
    y: usize,
    value: char,
    g: usize,
    h: usize,
}

fn search_path(field: &str, start: &Cell, draw: bool) -> Option<usize> {
    let field: Vec<_> = field.lines().collect();
    let end = field
        .iter()
        .enumerate()
        .find_map(|(y, line)| Some((y, line.find('E')?)))?;
    let end = Cell {
        x: end.1,
        y: end.0,
        value: 'z',
        g: 0,
        h: 0,
    };
    let height = field.len();
    let width = field[0].len();
    if draw {
        println!("width: {width}, height: {height}");
    }

    let mut vis_field = vec![" ".repeat(width); height];
    unsafe {
        vis_field[start.y].as_bytes_mut()[start.x] = 'S' as _;
        vis_field[end.y].as_bytes_mut()[end.x] = 'E' as _;
    }
    if draw {
        draw_vis_field(&vis_field);
    }

    let mut open = VecDeque::new();
    open.push_back(start.clone());
    let mut closed: HashSet<(usize, usize)> = HashSet::new();

    let mut counter = 0;
    while let Some(old_cell) = open.pop_front() {
        counter += 1;
        if counter % 100 == 0 && draw {
            draw_vis_field(&vis_field);
            println!("counter = {counter:>8}, popped: {old_cell:>3?}")
        }
        let neighbors = [(-1, 0, 'v'), (1, 0, '^'), (0, -1, '>'), (0, 1, '<')];
        for (y, x, direction_char) in neighbors {
            let y = <usize>::try_from(old_cell.y as isize + y).ok();
            let x = <usize>::try_from(old_cell.x as isize + x).ok();
            if let Some((x, y, new_char)) = x
                .zip(y)
                .and_then(|(x, y)| Some((x, y, field.get(y).and_then(|line| line.chars().nth(x))?)))
            {
                if new_char == 'E' && (old_cell.value == 'y' || old_cell.value == 'z') {
                    if draw {
                        draw_vis_field(&vis_field);
                    }
                    return Some(old_cell.g + 1);
                }
                let diff = new_char as i32 - old_cell.value as i32;
                if diff > 1 {
                    continue;
                }

                let new_cell = Cell {
                    x,
                    y,
                    value: new_char,
                    g: old_cell.g + 1,
                    h: end.x.abs_diff(x) + end.y.abs_diff(y),
                };

                if closed.contains(&(x, y)) {
                    continue;
                } else if let Some(existing_pos) = open
                    .iter()
                    .position(|open| open.x == new_cell.x && open.y == new_cell.y)
                {
                    if open[existing_pos].g > new_cell.g {
                        open.remove(existing_pos);
                        let pos =
                            open.partition_point(|cell| cell.g + cell.h < new_cell.g + new_cell.h);
                        open.insert(pos, new_cell);
                    } else {
                        continue;
                    }
                } else {
                    let pos =
                        open.partition_point(|cell| cell.g + cell.h < new_cell.g + new_cell.h);
                    open.insert(pos, new_cell);
                }
                unsafe {
                    vis_field[y].as_bytes_mut()[x] = direction_char as _;
                }
            }
        }
        closed.insert((old_cell.x, old_cell.y));
    }

    if draw {
        draw_vis_field(&vis_field);
    }
    None
}

fn draw_vis_field(vis_field: &Vec<String>) {
    println!("+{}+", "-".repeat(vis_field[0].len()));
    for line in vis_field {
        println!("|{line}|");
    }
    println!("+{}+", "-".repeat(vis_field[0].len()));
}
