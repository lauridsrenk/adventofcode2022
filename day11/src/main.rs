#![feature(iter_next_chunk)]
use std::{env, fs};

fn main() {
    let mut args = env::args().skip(1);
    let file = args.next().unwrap();
    let content = fs::read_to_string(file).unwrap();

    let monkeys_1: Vec<_> = content
        .split("Monkey")
        .filter_map(|monkey| Monkey::try_new(monkey, operation_part1))
        .collect();

    let monkeys_2: Vec<_> = content
        .split("Monkey")
        .filter_map(|monkey| Monkey::try_new(monkey, operation_part2))
        .collect();
    // for monkey in &monkeys {
    // println!("Starting items: {:?}", monkey.items);
    // println!("Test: divisible by {}", monkey.test);
    // println!("  If true: throw to monkey {}", monkey.if_true);
    // println!("  If false: throw to monkey {}", monkey.if_false);
    // }

    println!("----- Part 1 -----");
    solve(monkeys_1, 20);
    println!("----- Part 2 -----");
    solve(monkeys_2, 10000);
}

fn solve(mut monkeys: Vec<Monkey>, rounds: usize) {
    let prod: usize = monkeys.iter().map(|monkey| monkey.test).product();
    for round in 1..=rounds {
        for idx in 0..monkeys.len() {
            // println!("Monkey {idx}:");
            let monkey = &mut monkeys[idx];
            let items: Vec<_> = monkey
                .items
                .drain(..)
                .map(&monkey.operation)
                .map(|item| item % prod)
                .collect();
            monkey.inspected += items.len();
            let Monkey {
                test,
                if_true,
                if_false,
                ..
            } = *monkey;
            for item in items {
                // println!("\tnew worry level is {item}");
                if item % test == 0 {
                    let new_idx = if_true;
                    // println!("\tworry level is divisible by {test}");
                    // println!("\titem is thrown to Monkey {new_idx}");
                    monkeys[new_idx].items.push(item);
                } else {
                    let new_idx = if_false;
                    // println!("\tworry level is not divisible by {test}");
                    // println!("\titem is thrown to Monkey {new_idx}");
                    monkeys[new_idx].items.push(item);
                }
            }
        }

        if round % 1000 == 0 || round == 20 {
            println!("round {round}");
            for idx in 0..monkeys.len() {
                println!(
                    "Monkey {idx} inspected items {} times",
                    monkeys[idx].inspected
                );
            }
        }
    }
    println!("---");
    for idx in 0..monkeys.len() {
        println!(
            "Monkey {idx} inspected items {} times",
            monkeys[idx].inspected
        );
    }
    let mut top: Vec<_> = monkeys.iter().map(|monkey| monkey.inspected).collect();
    top.sort();
    top.reverse();
    println!("Monkey Business: {}", top[0] * top[1]);
}

struct Monkey {
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: usize,
    if_true: usize,
    if_false: usize,
    inspected: usize,
}

impl Monkey {
    fn try_new<F>(monkey: &str, operation_function: F) -> Option<Monkey>
    where
        F: Fn(&str) -> Box<dyn Fn(usize) -> usize>,
    {
        let [_, items, operation, test, if_true, if_false] = monkey
            .lines()
            .next_chunk()
            .ok()?
            .map(|line| line.split_once(":").map(|w| w.1));
        let items = items?
            .split(",")
            .flat_map(|word| word.trim().parse())
            .collect();

        Some(Monkey {
            items,
            operation: (operation_function)(operation?),
            test: test?.rsplit_once(" ")?.1.parse().ok()?,
            if_true: if_true?.rsplit_once(" ")?.1.parse().ok()?,
            if_false: if_false?.rsplit_once(" ")?.1.parse().ok()?,
            inspected: 0,
        })
    }
}

fn operation_part1(operation: &str) -> Box<dyn Fn(usize) -> usize> {
    let [_, _, _, _, op, right] = operation.split(" ").next_chunk().unwrap();
    match (op, right.parse::<usize>()) {
        ("*", Ok(num)) => Box::new(move |old| (old * num) / 3),
        ("*", Err(..)) => Box::new(move |old| (old * old) / 3),
        ("+", Ok(num)) => Box::new(move |old| (old + num) / 3),
        ("+", Err(..)) => Box::new(move |old| (old + old) / 3),
        _ => panic!("operand is neither * nor +, but '{:?}'", op),
    }
}
fn operation_part2(operation: &str) -> Box<dyn Fn(usize) -> usize> {
    let [_, _, _, _, op, right] = operation.split(" ").next_chunk().unwrap();
    match (op, right.parse::<usize>()) {
        ("*", Ok(num)) => Box::new(move |old| old * num),
        ("*", Err(..)) => Box::new(move |old| old * old),
        ("+", Ok(num)) => Box::new(move |old| (old + num)),
        ("+", Err(..)) => Box::new(move |old| (old + old)),
        _ => panic!("operand is neither * nor +, but '{:?}'", op),
    }
}
