pub mod str {
    use std::str::FromStr;
    
    pub trait StringOps {
        fn try_split_once<'a>(&'a self, delimiter: &str) -> Result<(&'a str, &'a str), String>;
    }

    impl StringOps for str { 
        fn try_split_once<'a>(&'a self, delimiter: &str) -> Result<(&'a str, &'a str), String> {
            self.split_once(delimiter)
                .ok_or(format!("failed to split \"{self}\" by \"{delimiter}\""))
        }
    }

    pub fn parse_sequence<T: FromStr>(line: &str) -> Vec<T> where T::Err: std::fmt::Debug {
        line
            .split_whitespace()
            .map(|s| s.parse::<T>().unwrap())
            .collect()
    }
}

use std::time::Instant;
use std::error::Error;
use std::fs;

pub type Result<T, E = Box<dyn Error>> = core::result::Result<T, E>;

pub enum Part {
    One,
    Two
}

pub fn run<T: std::fmt::Debug>(file_path: &str, part: Part, solve: impl Fn(&str, Part) -> Result<T>) {
    let time = Instant::now();
    let part_str = match part {
        Part::One => "part1",
        Part::Two => "part2"
    };
    let content = fs::read_to_string(file_path).unwrap();
    println!("Running for \"{file_path}\" ({part_str}):");
    println!("{:?} ({} µs)", solve(&content, part), time.elapsed().as_micros());
    println!();
}
