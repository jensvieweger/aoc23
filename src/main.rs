mod days;
mod utils;

use crate::days::day01::*;
use crate::utils::*;


fn main() {
    //println!("Hello, world!");
    //println!("{:?}", read_lines());

    let cal_vals = extract_calvals(&read_lines());
    let sum:u32 = cal_vals.iter().map(|&b| b as u32).sum();
    println!("{:?}", sum);
}

