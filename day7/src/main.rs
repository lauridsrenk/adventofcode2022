use std::str::FromStr;
use std::{env, fs};

fn main() {
    let mut args = env::args().skip(1);
    let part = args.next().unwrap();
    let file = args.next().unwrap();
    let content = fs::read_to_string(file).unwrap();

    let dirs = vec![("/".into(), 0)];
    let mut dirs: Vec<(String, u32)> =
        content.trim().lines().skip(1).fold(dirs, |mut dirs, line| {
            // line starts with number, add to tally
            if let Some(num) = line.split(' ').flat_map(u32::from_str).next() {
                let len = dirs.len() - 2;
                dirs[len].1 += num;
            }
            // ls; prepare a new path
            else if line == "$ ls" {
                let name = dirs.last().unwrap().0.clone();
                dirs.push((name, 0));
            }
            // cd ..; go up one segment
            else if line == "$ cd .." {
                let path = &mut dirs.last_mut().unwrap().0;
                let idx = path.rfind('/').unwrap();
                path.truncate(idx);
            }
            // cd <name>; add new segment to path
            else if line.starts_with("$ cd") {
                let name = line.rsplit_once(" ").unwrap().1;
                let dir = &mut dirs.last_mut().unwrap();
                dir.0 += "/";
                dir.0 += name;
            }
            dirs
        });
    dirs.pop();

    for idx in (1..dirs.len()).rev() {
        let (name, size) = &dirs[idx];
        let size = *size;
        let parent_idx = dirs[0..idx]
            .iter()
            .rposition(|(parent, _)| name.starts_with(parent))
            .unwrap();
        dirs[parent_idx].1 += size;
    }

    if part == "part1" {
        let sum: u32 = dirs
            .into_iter()
            .filter_map(|(_, size)| (size <= 100_000).then_some(size))
            .sum();
        println!("total sum: {sum}");
    } else if part == "part2" {
        let used = dirs[0].1;
        let free = 70_000_000 - used;
        let needed = 30_000_000 - free;
        println!("needed: {needed}");

        let (name, space) = dirs
            .into_iter()
            .filter(|(_, size)| *size >= needed)
            //.inspect(|(name, size)| println!("{name:<50}: {size:>10}"))
            .min_by_key(|(_, size)| *size)
            .unwrap();
        let name = name.trim_start_matches('/');
        println!("smallest dir over needed, {name}, frees {space}");
    }
}
