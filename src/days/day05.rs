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
    seed2soil: Map,
    soil2fert: Map,
    fert2water: Map,
    water2light: Map,
    light2temp: Map,
    temp2humid: Map,
    humid2loc: Map
}

fn parse_seeds(data: &String) -> Option<Vec<u32>> {
    let mut seeds = Vec::new();

    let seed_str: Vec<_> = data.split(": ").collect();

    for seed in seed_str[1].split(" ") {
        let seed_nr: u32 = seed.parse().unwrap();
        seeds.push(seed_nr);
    }

    Some(seeds)
}

fn parse_key(data: &String) -> Option<Key> {
    let key_parts: Vec<_> = data.split(" ").collect();

    let k: u32 = key_parts[0].parse().unwrap();
    let map_to: u32 = key_parts[1].parse().unwrap();
    let map_range: u32 = key_parts[2].parse().unwrap();

    Some(Key{k: k, map_to: map_to, map_range : map_range})
}

fn parse_map(data: &Vec<String>) -> Option<Map> {
    let mut mapdat = Vec::new();

    for line in data {
        let key = parse_key(line);
        if key.is_some() {
            mapdat.push(key.unwrap());
        }
    }
    Some(Map{data: mapdat})
}

fn parse_input(data: &Vec<String>) -> Option<Almanac> {
    
    let seeds = parse_seeds(&data[0]);

    let index_seed2soil = data.iter().position(|r| r == "seed-to-soil map:").unwrap();
    let index_soil2fert = data.iter().position(|r| r == "soil-to-fertilizer map:").unwrap();
    let index_fert2water = data.iter().position(|r| r == "fertilizer-to-water map:").unwrap();
    let index_water2light = data.iter().position(|r| r == "water-to-light map:").unwrap();
    let index_light2temp = data.iter().position(|r| r == "light-to-temperature map:").unwrap();
    let index_temp2humid = data.iter().position(|r| r == "temperature-to-humidity map:").unwrap();
    let index_humid2loc = data.iter().position(|r| r == "humidity-to-location map:").unwrap();

    let seed2soil = parse_map(&data[index_seed2soil+1..index_soil2fert-1].to_vec());
    let soil2fert = parse_map(&data[index_soil2fert+1..index_fert2water-1].to_vec());
    let fert2water = parse_map(&data[index_fert2water+1..index_water2light-1].to_vec());
    let water2light = parse_map(&data[index_water2light+1..index_light2temp-1].to_vec());
    let light2temp = parse_map(&data[index_light2temp+1..index_temp2humid-1].to_vec());
    let temp2humid = parse_map(&data[index_temp2humid+1..index_humid2loc-1].to_vec());
    let humid2loc = parse_map(&data[index_humid2loc+1..].to_vec());

    if seeds.is_none() || seed2soil.is_none() || soil2fert.is_none() || fert2water.is_none() ||
        water2light.is_none() || light2temp.is_none() || temp2humid.is_none() || humid2loc.is_none() {
            return None;
        }

    Some(Almanac {
        seeds: seeds.unwrap(),
        seed2soil: seed2soil.unwrap(),
        soil2fert: soil2fert.unwrap(),
        fert2water: fert2water.unwrap(),
        water2light: water2light.unwrap(),
        light2temp: light2temp.unwrap(),
        temp2humid: temp2humid.unwrap(),
        humid2loc: humid2loc.unwrap() })
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
    fn test_parse_seeds() {
        let data = fill_input();

        let seeds = parse_seeds(&data[0]);

        assert_eq!(seeds.is_none(), false);
        let seedsv = seeds.unwrap();
        assert_eq!(seedsv.len(), 4);

        assert_eq!(seedsv[0], 79);
        assert_eq!(seedsv[1], 14);
        assert_eq!(seedsv[2], 55);
        assert_eq!(seedsv[3], 13);
    }

    #[test]
    fn test_parse_key() {
        let data = fill_input();

        let key = parse_key(&data[3]);

        assert_eq!(key.is_none(), false);
        let key_data = key.unwrap();

        assert_eq!(key_data.k, 50);
        assert_eq!(key_data.map_to, 98);
        assert_eq!(key_data.map_range, 2);
    }

    #[test]
    fn test_parse_map() {
        let data = fill_input();

        let map = parse_map(&data[3..=4].to_vec());

        assert_eq!(map.is_none(), false);
        let map_data = map.unwrap();
        assert_eq!(map_data.data.len(), 2);

        assert_eq!(map_data.data[0].k, 50);
        assert_eq!(map_data.data[0].map_to, 98);
        assert_eq!(map_data.data[0].map_range, 2);

        assert_eq!(map_data.data[1].k, 52);
        assert_eq!(map_data.data[1].map_to, 50);
        assert_eq!(map_data.data[1].map_range, 48);
    }

    #[test]
    fn test_parse_input() {
        let data = fill_input();

        let almanc = parse_input(&data);

        assert_eq!(almanc.is_none(), false);
        let alm_dat = almanc.unwrap();

        assert_eq!(alm_dat.seeds.len(), 4);
        assert_eq!(alm_dat.seeds[0], 79);
        assert_eq!(alm_dat.seeds[1], 14);
        assert_eq!(alm_dat.seeds[2], 55);
        assert_eq!(alm_dat.seeds[3], 13);

        assert_eq!(alm_dat.seed2soil.data.len(), 2);
        assert_eq!(alm_dat.seed2soil.data[0].k, 50);
        assert_eq!(alm_dat.seed2soil.data[0].map_to, 98);
        assert_eq!(alm_dat.seed2soil.data[0].map_range, 2);

        assert_eq!(alm_dat.seed2soil.data[1].k, 52);
        assert_eq!(alm_dat.seed2soil.data[1].map_to, 50);
        assert_eq!(alm_dat.seed2soil.data[1].map_range, 48);

        assert_eq!(alm_dat.soil2fert.data.len(), 3);
        assert_eq!(alm_dat.soil2fert.data[0].k, 0);
        assert_eq!(alm_dat.soil2fert.data[0].map_to, 15);
        assert_eq!(alm_dat.soil2fert.data[0].map_range, 37);
        assert_eq!(alm_dat.soil2fert.data[1].k, 37);
        assert_eq!(alm_dat.soil2fert.data[1].map_to, 52);
        assert_eq!(alm_dat.soil2fert.data[1].map_range, 2);
        assert_eq!(alm_dat.soil2fert.data[2].k, 39);
        assert_eq!(alm_dat.soil2fert.data[2].map_to, 0);
        assert_eq!(alm_dat.soil2fert.data[2].map_range, 15);

        assert_eq!(alm_dat.fert2water.data.len(), 4);
        assert_eq!(alm_dat.fert2water.data[0].k, 49);
        assert_eq!(alm_dat.fert2water.data[0].map_to, 53);
        assert_eq!(alm_dat.fert2water.data[0].map_range, 8);
        assert_eq!(alm_dat.fert2water.data[1].k, 0);
        assert_eq!(alm_dat.fert2water.data[1].map_to, 11);
        assert_eq!(alm_dat.fert2water.data[1].map_range, 42);
        assert_eq!(alm_dat.fert2water.data[2].k, 42);
        assert_eq!(alm_dat.fert2water.data[2].map_to, 0);
        assert_eq!(alm_dat.fert2water.data[2].map_range, 7);
        assert_eq!(alm_dat.fert2water.data[3].k, 57);
        assert_eq!(alm_dat.fert2water.data[3].map_to, 7);
        assert_eq!(alm_dat.fert2water.data[3].map_range, 4);

        assert_eq!(alm_dat.water2light.data.len(), 2);
        assert_eq!(alm_dat.water2light.data[0].k, 88);
        assert_eq!(alm_dat.water2light.data[0].map_to, 18);
        assert_eq!(alm_dat.water2light.data[0].map_range, 7);
        assert_eq!(alm_dat.water2light.data[1].k, 18);
        assert_eq!(alm_dat.water2light.data[1].map_to, 25);
        assert_eq!(alm_dat.water2light.data[1].map_range, 70);

        assert_eq!(alm_dat.light2temp.data.len(), 3);
        assert_eq!(alm_dat.light2temp.data[0].k, 45);
        assert_eq!(alm_dat.light2temp.data[0].map_to, 77);
        assert_eq!(alm_dat.light2temp.data[0].map_range, 23);
        assert_eq!(alm_dat.light2temp.data[1].k, 81);
        assert_eq!(alm_dat.light2temp.data[1].map_to, 45);
        assert_eq!(alm_dat.light2temp.data[1].map_range, 19);
        assert_eq!(alm_dat.light2temp.data[2].k, 68);
        assert_eq!(alm_dat.light2temp.data[2].map_to, 64);
        assert_eq!(alm_dat.light2temp.data[2].map_range, 13);

        assert_eq!(alm_dat.temp2humid.data.len(), 2);
        assert_eq!(alm_dat.temp2humid.data[0].k, 0);
        assert_eq!(alm_dat.temp2humid.data[0].map_to, 69);
        assert_eq!(alm_dat.temp2humid.data[0].map_range, 1);
        assert_eq!(alm_dat.temp2humid.data[1].k, 1);
        assert_eq!(alm_dat.temp2humid.data[1].map_to, 0);
        assert_eq!(alm_dat.temp2humid.data[1].map_range, 69);

        assert_eq!(alm_dat.humid2loc.data.len(), 2);
        assert_eq!(alm_dat.humid2loc.data[0].k, 60);
        assert_eq!(alm_dat.humid2loc.data[0].map_to, 56);
        assert_eq!(alm_dat.humid2loc.data[0].map_range, 37);
        assert_eq!(alm_dat.humid2loc.data[1].k, 56);
        assert_eq!(alm_dat.humid2loc.data[1].map_to, 93);
        assert_eq!(alm_dat.humid2loc.data[1].map_range, 4);
    }

}