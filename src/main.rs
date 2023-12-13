mod days;
mod utils;

use crate::days::day01::*;
use crate::days::day02::*;
use crate::utils::*;

use std::env;

fn day01() {
    let cal_vals = extract_calvals(&read_lines());
    let sum:u32 = cal_vals.iter().map(|&b| b as u32).sum();
    println!("{:?}", sum);

}

fn day02() {
    let games = parse_games(&read_lines()).unwrap();

    let bag = Bag{ red: 12, green: 13, blue: 14};
    let valid_games = calc_valid_games(games, bag).unwrap();
    let sum:u32 = valid_games.iter().map(|&b| b as u32).sum();
    println!("{:?}", sum);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "01" => day01(),
        "02" => day02(),
        _ => return
    };
}

