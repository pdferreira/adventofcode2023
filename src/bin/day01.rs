use std::error::Error;
use std::fs;
use itertools::process_results;

type Result<T> = core::result::Result<T, Box<dyn Error>>;

fn ok<T>(v: T) -> Result<T> {
    Result::Ok(v)
}

const DIGIT_NAMES: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main() {
    println!("example (part1): {:?}", solve("inputs/day01_example", parse_digits_part1));
    println!("input (part1): {:?}", solve("inputs/day01", parse_digits_part1));
    println!("example (part2): {:?}", solve("inputs/day01_example2", parse_digits_part2));
    println!("input (part2): {:?}", solve("inputs/day01", parse_digits_part2));
}

fn solve(path: &str, parse_digits: fn(&str) -> Result<Vec<char>>) -> Result<u32> {
    let content = fs::read_to_string(path).unwrap();
    let res = process_results(
        content
        .lines()
        .map(|l| {
            let digits = parse_digits(l)?;
            let num = format!("{}{}", digits[0], digits[digits.len() - 1]).parse::<u32>()?;
            return ok(num);
        }),
        |it| it.sum::<u32>())?;
    return Ok(res);
}

fn parse_digits_part1(line: &str) -> Result<Vec<char>> {
    return Ok(line.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>());
}

fn parse_digits_part2(line: &str) -> Result<Vec<char>> {
    let first = get_first_digit(line, false).ok_or(format!("No digit found"))?;
    let last = get_first_digit(line, true).ok_or(format!("No digit found"))?;
    return ok(vec![first, last]);
}

fn get_first_digit(line: &str, from_end: bool) -> Option<char> {
    let line_it: Vec<(usize, char)> = if from_end { 
        line.char_indices().rev().collect::<Vec<(usize, char)>>()
    } else { 
        line.char_indices().collect::<Vec<(usize, char)>>() 
    };

    for (i, c) in line_it {
        if c.is_numeric() {
            return Some(c);
        }
        
        for (name, digit) in DIGIT_NAMES.iter().zip(1..10) {
            if line[i..].starts_with(name) {
                return char::from_digit(digit, 10);
            }
        }
    }
    
    return None
}