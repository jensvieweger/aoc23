
#[derive(Copy, Clone, Debug)]
struct Set {
    red: usize,
    green: usize,
    blue: usize
}

#[derive(Clone, Debug)]
pub struct Game {
    index: usize,
    sets: Vec<Set>
}

pub struct Bag {
    pub red: usize,
    pub green: usize,
    pub blue: usize
}

fn parse_set(str: &str) -> Option<Set> {
    let part_it = str.split(", ");

    let mut set = Set{ red: 0, green: 0,blue: 0};

    for part in part_it {
        //println!("part {:?}", part);
        let count_n_col:Vec<_> = part.split(" ").collect();
        //println!("0: {:?}", count_n_col[0]);
        let count = usize::from_str_radix(count_n_col[0], 10).unwrap();
        //println!("0-: {:?}", count);
        match count_n_col[1] {
            "red" => set.red = count,
            "green" => set.green = count,
            "blue" => set.blue = count,
            _ => return None
        }
    }
    Some(set)
}

fn parse_game(str: &String) -> Option<Game> {
    let part: Vec<_> = str.split(": ").collect();

    //println!("0: {:?}", part[0].split(" ").collect::<Vec<_>>()[1]);
    let index = usize::from_str_radix(part[0].split(" ").collect::<Vec<_>>()[1], 10).unwrap();
    let mut game = Game { index: index, sets: Vec::new()};

    for setstr in part[1].split("; ") {
        let set = parse_set(setstr);
        if set.is_none() {
            return None
        }
        game.sets.push(set.unwrap());
    }

    Some(game)
}

pub fn parse_games(data: &Vec<String>) -> Option<Vec<Game>> {
    let mut result: Vec<Game> = Vec::new();

    for line in data {
        let game = parse_game(line).unwrap();

        result.push(game);
    }

    Some(result)
}

pub fn calc_valid_games(games: Vec<Game>, bag: Bag) -> Option<Vec<usize>> {
    let mut result: Vec<usize> = Vec::new();

    for game in games {
        let mut game_is_valid = true;

        for set in game.sets {
            if set.red > bag.red {
                game_is_valid = false;
            }
            if set.green > bag.green {
                game_is_valid = false;
            }
            if set.blue > bag.blue {
                game_is_valid = false;
            }

            if !game_is_valid {
                break;
            }
        }
        if game_is_valid {
            result.push(game.index);
        }

    }
    Some(result)
}

pub fn calc_powers(games: Vec<Game>) -> Option<Vec<u32>> {
    let mut result: Vec<u32> = Vec::new();

    for game in games {
        let mut min_r = 0;
        let mut min_g = 0;
        let mut min_b = 0;

        for set in game.sets {
            if set.red > min_r {
                min_r = set.red;
            }
            if set.green > min_g {
                min_g = set.green;
            }
            if set.blue > min_b {
                min_b = set.blue;
            }
        }
        result.push((min_r as u32) * (min_g as u32) * (min_b as u32));

    }
    Some(result)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_set() {
        let set1 = parse_set(&"3 blue, 4 red".to_string());

        assert_eq!(set1.is_none(), false);
        assert_eq!(set1.unwrap().red, 4);
        assert_eq!(set1.unwrap().green, 0);
        assert_eq!(set1.unwrap().blue, 3);

        let set2 = parse_set(&"1 red, 2 green, 6 blue".to_string());

        assert_eq!(set2.is_none(), false);
        assert_eq!(set2.unwrap().red, 1);
        assert_eq!(set2.unwrap().green, 2);
        assert_eq!(set2.unwrap().blue, 6);

        let set3 = parse_set(&"2 green".to_string());

        assert_eq!(set3.is_none(), false);
        assert_eq!(set3.unwrap().red, 0);
        assert_eq!(set3.unwrap().green, 2);
        assert_eq!(set3.unwrap().blue, 0);
    }

    #[test]
    fn test_parse_game() {
        let game = parse_game(&"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string());

        assert_eq!(game.is_none(), false);
        assert_eq!(game.as_ref().unwrap().index, 1);

        let set1 = game.as_ref().unwrap().sets[0];

        assert_eq!(set1.red, 4);
        assert_eq!(set1.green, 0);
        assert_eq!(set1.blue, 3);

        let set2 = game.as_ref().unwrap().sets[1];

        assert_eq!(set2.red, 1);
        assert_eq!(set2.green, 2);
        assert_eq!(set2.blue, 6);

        let set3 = game.as_ref().unwrap().sets[2];

        assert_eq!(set3.red, 0);
        assert_eq!(set3.green, 2);
        assert_eq!(set3.blue, 0);
    }

    #[test]
    fn test_parse_games() {
        let mut lines = Vec::new();

        lines.push("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string());
        lines.push("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string());
        lines.push("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string());
        lines.push("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string());
        lines.push("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string());

        let games = parse_games(&lines);
        assert_eq!(games.is_none(), false);
        assert_eq!(games.unwrap().len(), 5);
    }

    #[test]
    fn test_calc_valid_games() {
        let mut lines = Vec::new();

        lines.push("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string());
        lines.push("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string());
        lines.push("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string());
        lines.push("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string());
        lines.push("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string());

        let games = parse_games(&lines).unwrap();
        
        let bag = Bag { red: 12, green: 13, blue: 14};

        let valid_games = calc_valid_games(games, bag);
        assert_eq!(valid_games.is_none(), false);
        assert_eq!(valid_games.as_ref().unwrap().len(), 3);
        assert_eq!(valid_games.as_ref().unwrap()[0], 1);
        assert_eq!(valid_games.as_ref().unwrap()[1], 2);
        assert_eq!(valid_games.as_ref().unwrap()[2], 5);
    }

    #[test]
    fn test_calc_powers() {
        let mut lines = Vec::new();

        lines.push("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string());
        lines.push("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string());
        lines.push("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string());
        lines.push("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string());
        lines.push("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string());

        let games = parse_games(&lines).unwrap();

        let powers = calc_powers(games);
        assert_eq!(powers.is_none(), false);
        assert_eq!(powers.as_ref().unwrap().len(), 5);
        assert_eq!(powers.as_ref().unwrap()[0], 48);
        assert_eq!(powers.as_ref().unwrap()[1], 12);
        assert_eq!(powers.as_ref().unwrap()[2], 1560);
        assert_eq!(powers.as_ref().unwrap()[3], 630);
        assert_eq!(powers.as_ref().unwrap()[4], 36);
    }
}