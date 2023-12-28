struct Key {
    k: u32,
    map_to: u32,
    map_range: u32
}

struct Map {
    data: Vec<Key>
}

struct Almanac {
    seeds: Vec<u32>,
    seed2soil: Vec<Map>,
    soil2fert: Vec<Map>,
    fert2water: Vec<Map>,
    water2light: Vec<Map>,
    light2temp: Vec<Map>,
    temp2humid: Vec<Map>,
    humid2loc: Vec<Map>
}

fn parse_input(data: &Vec<String>) -> Option<Almanac> {
    let mut seeds = Vec::new();

    let index_seed2soil = data.iter().position(|r| r == "seed-to-soil map:").unwrap();
    let index_soil2fert = data.iter().position(|r| r == "soil-to-fertilizer map:").unwrap();
    let index_fert2water = data.iter().position(|r| r == "fertilizer-to-water map:").unwrap();
    let index_water2light = data.iter().position(|r| r == "water-to-light map:").unwrap();
    let index_light2temp = data.iter().position(|r| r == "light-to-temperature map:").unwrap();
    let index_temp2humid = data.iter().position(|r| r == "temperature-to-humidity map:").unwrap();
    let index_humid2loc = data.iter().position(|r| r == "humidity-to-location map:").unwrap();
    None
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn fill_input() -> Vec<String> {
        vec![
            "seeds: 79 14 55 13".to_string(),
            "".to_string(),
            "seed-to-soil map:".to_string(),
            "50 98 2".to_string(),
            "52 50 48".to_string(),
            "".to_string(),
            "soil-to-fertilizer map:".to_string(),
            "0 15 37".to_string(),
            "37 52 2".to_string(),
            "39 0 15".to_string(),
            "".to_string(),
            "fertilizer-to-water map:".to_string(),
            "49 53 8".to_string(),
            "0 11 42".to_string(),
            "42 0 7".to_string(),
            "57 7 4".to_string(),
            "".to_string(),
            "water-to-light map:".to_string(),
            "88 18 7".to_string(),
            "18 25 70".to_string(),
            "".to_string(),
            "light-to-temperature map:".to_string(),
            "45 77 23".to_string(),
            "81 45 19".to_string(),
            "68 64 13".to_string(),
            "".to_string(),
            "temperature-to-humidity map:".to_string(),
            "0 69 1".to_string(),
            "1 0 69".to_string(),
            "".to_string(),
            "humidity-to-location map:".to_string(),
            "60 56 37".to_string(),
            "56 93 4".to_string()]
    }

    #[test]
    fn test_parse_input() {
        let data = fill_input();

        let almanc = parse_input(&data);

        assert_eq!(almanc.is_none(), false);

    }

}