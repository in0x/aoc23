use super::common;
use std::fs;
use std::path;

struct MatchTo {
    token: Vec<char>,
    head: usize,
    yields: u32,
}

impl MatchTo {
    fn new(string: &str, value: u32) -> MatchTo {
        MatchTo {
            token: string.chars().collect(),
            head: 0,
            yields: value
        }
    }

    pub fn reset(&mut self) {
        self.head = 0
    }

    fn is_next(&mut self, c: char) -> bool {
        if c.is_ascii_digit() && c.to_digit(10).unwrap() == self.yields {
            self.head = self.token.len();
            return true;
        }

        if c == self.token[self.head] {
            self.head += 1;
            return true;
        }

        false
    }

    pub fn is_complete(&self) -> bool {
        self.token.len() == self.head
    }
}

pub fn run(input: &path::PathBuf) {
    let mut tokens = vec![
        MatchTo::new("one",   1),
        MatchTo::new("two",   2),
        MatchTo::new("three", 3),
        MatchTo::new("four",  4),
        MatchTo::new("five",  5),
        MatchTo::new("six",   6),
        MatchTo::new("seven", 7),
        MatchTo::new("eight", 8),
        MatchTo::new("nine",  9),
    ];

    let mut sum = 0;
    let mut digits = Vec::new();
    let file_contents = fs::read_to_string(input).unwrap();
    for line in file_contents.split('\n') {
        if line.is_empty() { continue; }
        for c in line.chars() {
            for token in &mut tokens {
                if !token.is_next(c.to_ascii_lowercase()) {
                    // Try again, incase the word starts over on this letter
                    token.reset();
                    token.is_next(c.to_ascii_lowercase());
                }
               
                if token.is_complete() {
                    digits.push(token.yields);
                    token.reset();
                }
            }
        }

        sum += digits.first().unwrap() * 10 + digits.last().unwrap();
        for token in &mut tokens { token.reset(); }
        digits.clear();
    }

    println!("Sum: {}", sum);
}
