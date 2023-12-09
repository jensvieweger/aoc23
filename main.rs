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

#[derive(Copy, Clone, Debug)]
struct DigInd {
    index: usize,
    digit: i8
}

fn get_first_digit_word(str: &String) -> Option<DigInd> {
    let mut word_indices = Vec::new();

    word_indices.push(str.find("one"));
    word_indices.push(str.find("two"));
    word_indices.push(str.find("three"));
    word_indices.push(str.find("four"));
    word_indices.push(str.find("five"));
    word_indices.push(str.find("six"));
    word_indices.push(str.find("seven"));
    word_indices.push(str.find("eight"));
    word_indices.push(str.find("nine"));

    let word_index = word_indices.iter().fold( str.len()+1, |acc, x| {
        if x.is_some_and(|x| x<acc) { 
            x.unwrap()
        } else {
            acc
        }
    });

    if word_index > str.len() { return None;}

    let index = word_indices.iter().position(|&r| r.is_some_and(|x| x == word_index) ).unwrap();
    return Some(DigInd { index: (word_index), digit: ((index+1) as i8) })
    ;

}

fn get_last_digit_word(str: &String) -> Option<DigInd> {
    let mut word_indices = Vec::new();

    word_indices.push(str.rfind("one"));
    word_indices.push(str.rfind("two"));
    word_indices.push(str.rfind("three"));
    word_indices.push(str.rfind("four"));
    word_indices.push(str.rfind("five"));
    word_indices.push(str.rfind("six"));
    word_indices.push(str.rfind("seven"));
    word_indices.push(str.rfind("eight"));
    word_indices.push(str.rfind("nine"));

    let word_index = word_indices.iter().fold( 0, |acc, x| {
        if x.is_some_and(|x| x>acc) { 
            x.unwrap()
        } else {
            acc
        }
    });

    // assume that there is no input string containing only a single number-word
    if word_index == 0 { return None;}

    let index = word_indices.iter().position(|&r| r.is_some_and(|x| x == word_index) ).unwrap();
    return Some(DigInd { index: (word_index), digit: ((index+1) as i8) })
    ;

}

fn get_first_digit_char(str: &String) -> Option<DigInd> {
    let digit_index = str.find(char::is_numeric);
    if digit_index.is_none() {
        return None;
    }
    let digit: i8 = 
        match str.chars().nth(digit_index.unwrap()){
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
                
           };
    return Some(DigInd { index: (digit_index.unwrap()), digit: (digit) })
}

fn get_last_digit_char(str: &String) -> Option<DigInd> {
    let result = get_first_digit_char(&str.chars().rev().collect::<String>());
    if result.is_none() {return None;}
    return Some(DigInd { index: (str.len()-1-result.unwrap().index), digit: (result.unwrap().digit) })
}

fn get_first_digit(str: &String) -> i8 {
    let digit_word = get_first_digit_word(str);
    let digit_char = get_first_digit_char(str);

    let mut using = 2; // 0 = word; 1 = digit; 2 = none
    if digit_word.is_some() {
        using = 0;
        if digit_char.is_some_and(|x| x.index < digit_word.unwrap().index) {
                using = 1;
        }
    } else if digit_char.is_some() {
        using = 1;        
    }

    if using == 0 {
        return digit_word.unwrap().digit;
    } else if using == 1 {
        return digit_char.unwrap().digit;
    } else {0}
}

fn get_last_digit(str: &String) -> i8 {
    let digit_word = get_last_digit_word(str);
    let digit_char = get_last_digit_char(str);

    let mut using = 2; // 0 = word; 1 = digit; 2 = none
    if digit_word.is_some() {
        using = 0;
        if digit_char.is_some_and(|x| x.index > digit_word.unwrap().index) {
                using = 1;
        }
    } else if digit_char.is_some() {
        using = 1;        
    }

    if using == 0 {
        return digit_word.unwrap().digit;
    } else if using == 1 {
        return digit_char.unwrap().digit;
    } else {0}
}

fn extract_calvals(data: &Vec<String>) -> Vec<i8> {
    let mut result = Vec::new();

    for str in data {
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
        assert_eq!(get_first_digit(&"eightwothree".to_string()), 8);
    }

    #[test]
    fn test_get_last_digit() {
        assert_eq!(get_last_digit(&"asd3as4d".to_string()), 4);
        assert_eq!(get_last_digit(&"abcone2threexyz".to_string()), 3);
    }

    #[test]
    fn test_get_first_digit_word() {
        assert!(get_first_digit_word(&"asd3as4d".to_string()).is_none());
        let ret = get_first_digit_word(&"eightwothree".to_string());
        assert!(ret.is_some());
        assert_eq!(ret.unwrap().index, 0);
        assert_eq!(ret.unwrap().digit, 8);
    }

    #[test]
    fn test_get_last_digit_word() {
        assert!(get_last_digit_word(&"asd3as4d".to_string()).is_none());
        let ret = get_last_digit_word(&"abcone2threexyz".to_string());
        assert!(ret.is_some());
        assert_eq!(ret.unwrap().index, 7);
        assert_eq!(ret.unwrap().digit, 3);
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

        lines.clear();
        // second half of day one
        lines.push("two1nine".to_string());
        lines.push("eightwothree".to_string());
        lines.push("abcone2threexyz".to_string());
        lines.push("xtwone3four".to_string());
        lines.push("4nineeightseven2".to_string());
        lines.push("zoneight234".to_string());
        lines.push("7pqrstsixteen".to_string());

        let calvals2 = extract_calvals(&lines);

        assert_eq!(calvals2.len(), 7);
        assert_eq!(calvals2[0], 29);
        assert_eq!(calvals2[1], 83);
        assert_eq!(calvals2[2], 13);
        assert_eq!(calvals2[3], 24);
        assert_eq!(calvals2[4], 42);
        assert_eq!(calvals2[5], 14);
        assert_eq!(calvals2[6], 76);

        assert_eq!(calvals2.iter().map(|&b| b as u32).sum::<u32>(), 281u32);

    }
}