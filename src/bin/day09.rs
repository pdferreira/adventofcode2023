use std::fs;
use adventofcode2023::{Result, Part};
use adventofcode2023::str::parse_sequence;

fn main() {
    println!("example1 (part1): {:?}", solve("inputs/day09_example", Part::One));
    println!("input (part1): {:?}", solve("inputs/day09", Part::One));
    println!("example (part2): {:?}", solve("inputs/day09_example", Part::Two));
    println!("input (part2): {:?}", solve("inputs/day09", Part::Two));
}

fn solve(path: &str, part: Part) -> Result<i32> {
    let content = fs::read_to_string(path)?;
    let histories: Vec<_> = content
        .lines()
        .map(|l| parse_sequence::<i32>(l))
        .collect();

    Ok(match part {
        Part::One =>
            histories
                .iter()
                .map(|h| predict_next(&h))
                .sum(),
        
        Part::Two =>
            histories
                .iter()
                .map(|h| {
                    let reversed_h = h.iter().rev().map(|n| *n).collect();
                    predict_next(&reversed_h)
                })
                .sum()
    })
}

fn predict_next(seq: &Vec<i32>) -> i32 {
    let inner_seq: Vec<_> = seq
        .iter()
        .zip(seq.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect();

    if inner_seq.iter().all(|n| *n == 0) {
        seq[0]
    } else {
        seq.last().unwrap() + predict_next(&inner_seq)
    }
}
