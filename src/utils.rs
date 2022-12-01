use std::io::BufRead;
use std::str::FromStr;

pub struct Scanner<I: BufRead> {
    input: I,
    buffer: Vec<String>,
}

impl<I: BufRead> Scanner<I> {
    pub fn new(input: I) -> Self {
        Self {
            input,
            buffer: Vec::new(),
        }
    }

    pub fn next<T: FromStr>() -> T {
        todo!();
    }
}
