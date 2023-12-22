#[derive(Clone, Debug)]
pub struct Card {
    winning: Vec<u8>,
    own: Vec<u8>
}

pub fn parse_card(data: &String) -> Option<Card> {
    let split_colon:Vec<&str> = data.split(':').collect();
    let parts:Vec<&str> = split_colon[1].split('|').collect();

    let winning_str = parts[0].trim();
    let own_str = parts[1].trim();

    let mut winning = Vec::new();
    let mut own = Vec::new();

    for numstr in winning_str.split_whitespace() {
        let num = numstr.parse::<u8>().ok();
        if num.is_some() {
            winning.push(num.unwrap());
        }
    }

    for numstr in own_str.split_whitespace() {
        let num = numstr.parse::<u8>().ok();
        if num.is_some() {
            own.push(num.unwrap());
        }
    }

    Some (Card { winning : winning, own : own})
}

pub fn parse_cards(data: &Vec<String>) -> Option<Vec<Card>> {
    let mut cards = Vec::new();

    for line in data {
        let card = parse_card(line);
        if card.is_some() {
            cards.push(card.unwrap());
        }
    }
    Some(cards)
}

fn get_sum_winning_numbers_from_card(card: &Card) -> u32 {
    let mut count = 0;

    for own in card.own.iter() {
        /*let found = card.winning.iter().find(|&&x| x == own);
        if found.is_some() {
            sum = sum + u32::from(own.clone());
        }*/
        if card.winning.contains(own) {
            count = count + 1;
        }
    }
    if count == 0 {
        return 0;
    }
    u32::pow(2,count-1)
}

pub fn get_sum_cards(cards: &Vec<Card>) -> u32 {
    let mut sum = 0;
    for card in cards {
        sum = sum + get_sum_winning_numbers_from_card(card);
    }
    sum
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn fill_input() -> Vec<String> {
        let mut data = Vec::new();
        
        data.push("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string());
        data.push("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string());
        data.push("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string());
        data.push("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string());
        data.push("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string());
        data.push("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string());

        data
    }

    #[test]
    fn test_parse_card() {
        let data = fill_input();

        let card = parse_card(&data[0]);

        assert_eq!(card.is_none(), false);
        assert_eq!(card.as_ref().unwrap().winning.len(), 5);
        assert_eq!(card.as_ref().unwrap().winning[0], 41);
        assert_eq!(card.as_ref().unwrap().winning[1], 48);
        assert_eq!(card.as_ref().unwrap().winning[2], 83);
        assert_eq!(card.as_ref().unwrap().winning[3], 86);
        assert_eq!(card.as_ref().unwrap().winning[4], 17);

        assert_eq!(card.as_ref().unwrap().own.len(), 8);
        assert_eq!(card.as_ref().unwrap().own[0], 83);
        assert_eq!(card.as_ref().unwrap().own[1], 86);
        assert_eq!(card.as_ref().unwrap().own[2], 6);
        assert_eq!(card.as_ref().unwrap().own[3], 31);
        assert_eq!(card.as_ref().unwrap().own[4], 17);
        assert_eq!(card.as_ref().unwrap().own[5], 9);
        assert_eq!(card.as_ref().unwrap().own[6], 48);
        assert_eq!(card.as_ref().unwrap().own[7], 53);
    }

    #[test]
    fn test_parse_cards() {
        let data = fill_input();

        let cards = parse_cards(&data);

        assert_eq!(cards.is_none(), false);
        assert_eq!(cards.as_ref().unwrap().len(), 6);
    }

    #[test]
    fn test_get_sum_winning_numbers_from_card() {
        let mut winning = vec![41, 48, 83, 86, 17];
        let mut own = vec![83, 86, 6, 31, 17, 9, 48, 53];

        let card = Card { winning : winning, own : own};

        assert_eq!(get_sum_winning_numbers_from_card(&card), 8);
    }

    #[test]
    fn test_get_sum_cards() {
        let data = fill_input();

        let cards = parse_cards(&data);

        assert_eq!(cards.is_none(), false);
        assert_eq!(get_sum_cards(&cards.unwrap()), 13);
    }

}