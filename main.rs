use std::io::{self, BufRead};
use std::char;

fn read_lines() -> Vec<String> {
    let stdin = io::stdin();
    let mut result = Vec::new();

    for line in stdin.lock().lines() {
        result.push(line.unwrap().to_string())
    }
    result
}

fn get_first_digit(str: &String) -> i8 {
    let digit = str.chars().nth(str.find(char::is_numeric).unwrap());
    match digit{
        Some('1') => 1,
        Some('2') => 2,
        Some('3') => 3,
        Some('4') => 4,
        Some('5') => 5,
        Some('6') => 6,
        Some('7') => 7,
        Some('8') => 8,
        Some('9') => 9,
        Some('0') => 0,
        _ => 0
    }
}

fn get_last_digit(str: &String) -> i8 {
    return get_first_digit(&str.chars().rev().collect::<String>());
}

fn extract_calvals(data: &Vec<String>) -> Vec<i8> {
    let mut result = Vec::new();

    for str in data {
        println!("{:?}", str);
        let i = 10 * get_first_digit(str) + get_last_digit(str);

        result.push(i);
    }

    result
}

fn main() {
    //println!("Hello, world!");
    //println!("{:?}", read_lines());

    let cal_vals = extract_calvals(&read_lines());
    let sum:u32 = cal_vals.iter().map(|&b| b as u32).sum();
    println!("{:?}", sum);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_first_digit() {
        assert_eq!(get_first_digit(&"asd3as4d".to_string()), 3);
    }

    #[test]
    fn test_get_last_digit() {
        assert_eq!(get_last_digit(&"asd3as4d".to_string()), 4);
    }

    #[test]
    fn test_extract_calvals() {
        let mut lines = Vec::new();
        
        lines.push("1abc2".to_string());
        lines.push("pqr3stu8vwx".to_string());
        lines.push("a1b2c3d4e5f".to_string());
        lines.push("treb7uchet".to_string());

        let calvals = extract_calvals(&lines);

        assert_eq!(calvals.len(), 4);
        assert_eq!(calvals[0], 12);
        assert_eq!(calvals[1], 38);
        assert_eq!(calvals[2], 15);
        assert_eq!(calvals[3], 77);

        assert_eq!(calvals.iter().map(|&b| b as u32).sum::<u32>(), 142u32);
    }
}