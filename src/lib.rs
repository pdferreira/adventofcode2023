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

use std::error::Error;

pub type Result<T, E = Box<dyn Error>> = core::result::Result<T, E>;

pub enum Part {
    One,
    Two
}
