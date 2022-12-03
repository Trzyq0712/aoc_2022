use std::io::BufRead;
use std::str::FromStr;

pub struct Scanner<'a, I: BufRead> {
    input: &'a mut I,
    buffer: Vec<String>,
}

impl<'a, I: BufRead> Scanner<'a, I> {
    pub fn new(input: &'a mut I) -> Self {
        Self {
            input,
            buffer: Vec::new(),
        }
    }

    pub fn next<T: FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                break token.parse().ok().expect("Could not parse token");
            }
            let mut data = String::new();
            self.input
                .read_line(&mut data)
                .expect("Could not read line");
            self.buffer = data.split_ascii_whitespace().map(String::from).collect();
        }
    }
}
