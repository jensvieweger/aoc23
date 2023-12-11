use std::io::{self, BufRead};

pub fn read_lines() -> Vec<String> {
    let stdin = io::stdin();
    let mut result = Vec::new();

    for line in stdin.lock().lines() {
        result.push(line.unwrap().to_string())
    }
    result
}