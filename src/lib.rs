pub mod str {    
    pub trait StringOps {
        fn try_split_once<'a>(&'a self, delimiter: &str) -> Result<(&'a str, &'a str), String>;
    }

    impl StringOps for str { 
        fn try_split_once<'a>(&'a self, delimiter: &str) -> Result<(&'a str, &'a str), String> {
            self.split_once(delimiter)
                .ok_or(format!("failed to split \"{self}\" by \"{delimiter}\""))
        }
    }
}

use std::error::Error;

pub type Result<T, E = Box<dyn Error>> = core::result::Result<T, E>;

pub enum Part {
    One,
    Two
}
