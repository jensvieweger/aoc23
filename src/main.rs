mod days;
mod utils;

use crate::days::day01::*;
use crate::days::day02::*;
use crate::days::day03::*;
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

    //@TODO have game be borrowed...
    let valid_games = calc_valid_games(games.clone(), bag).unwrap();
    let sum:u32 = valid_games.iter().map(|&b| b as u32).sum();
    println!("sum valid games: {:?}", sum);

    let powers = calc_powers(games).unwrap();
    let powersum:u32 = powers.iter().sum();
    println!("sum valid powers: {:?}", powersum);

}

fn day03() {
    let data = read_lines();
    let partnumbers = get_part_numbers(&data).unwrap();
    let sum:u32 = partnumbers.iter().sum();
    println!("sum parts: {:?}", sum);

    let gearnumbers = get_gear_numbers(&data).unwrap();
    let gearsum:u32 = gearnumbers.iter().sum();
    println!("sum gears: {:?}", gearsum);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "01" => day01(),
        "02" => day02(),
        "03" => day03(),
        _ => return
    };
}

