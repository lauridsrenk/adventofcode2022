use serde::Deserialize;
use std::{cmp::Ordering, env, fmt, fs};

use Packet::*;

fn main() {
    let mut args = env::args().skip(1);
    let file = args.next().unwrap();
    let content = fs::read_to_string(file).unwrap();

    let content: &str = &content;
    let mut packets: Vec<_> = content
        .lines()
        .flat_map(serde_json::from_str::<Packet>)
        .collect();
    println!("----- Part 1 -----");
    let indices = packets
        .chunks(2)
        .enumerate()
        .filter_map(|(idx, pair)| (pair[0] < pair[1]).then_some(idx + 1))
        .inspect(|idx| print!("{idx}, "));
    let sum = indices.sum::<usize>();
    println!("\ntotal sum: {sum}");

    println!("----- Part 2 -----");
    let first_key = Multiple(vec![Multiple(vec![Single(2)])]);
    let second_key = Multiple(vec![Multiple(vec![Single(6)])]);
    packets.push(first_key.clone());
    packets.push(second_key.clone());
    packets.sort();
    // for (idx, packet) in packets.iter().enumerate() {
    // println!("{}: {packet}", idx + 1);
    // }
    let first = packets.iter().position(|v| v == &first_key).unwrap() + 1;
    let second = packets.iter().position(|v| v == &second_key).unwrap() + 1;
    println!("decoder indexes: {first}, {second}");
    println!("decoder key: {}", first * second);
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq, Ord)]
#[serde(untagged)]
enum Packet {
    Single(u8),
    Multiple(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        //println!("compare {self} vs {rhs}");
        match (self, rhs) {
            (Single(left), Single(right)) => left.partial_cmp(right),
            (Multiple(left), Multiple(right)) => match left.len().cmp(&right.len()) {
                Ordering::Less => left[..]
                    .partial_cmp(&right[..left.len()])
                    .map(|ord| match ord {
                        Ordering::Equal => Ordering::Less,
                        ord => ord,
                    }),
                Ordering::Equal => left.partial_cmp(right),
                Ordering::Greater => left[..right.len()].partial_cmp(right).map(|ord| match ord {
                    Ordering::Equal => Ordering::Greater,
                    ord => ord,
                }),
            },
            (Single(left), right) => Multiple(vec![Packet::Single(*left)]).partial_cmp(right),
            (left, Single(right)) => left.partial_cmp(&Multiple(vec![Packet::Single(*right)])),
        }
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Single(byte) => write!(formatter, "{byte}"),
            Multiple(packets) => {
                write!(formatter, "[")?;
                for packet in packets {
                    write!(formatter, "{packet}, ")?;
                }
                write!(formatter, "]")
            }
        }
    }
}
