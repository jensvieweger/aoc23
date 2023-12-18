#[derive(Copy, Clone, Debug)]
struct NumSym {
    line: usize,
    col: usize,
    len: usize
}

#[derive(Clone, Debug)]
struct NumbersAndSymbols {
    numbers: Vec<NumSym>,
    symbols: Vec<NumSym>
}

fn get_numbers_and_symbols(data: &Vec<String>) -> Option<NumbersAndSymbols> {
    let mut numbers: Vec<NumSym> = Vec::new();
    let mut symbols: Vec<NumSym> = Vec::new();

    let mut line_no = 0;
    for line in data {
        let mut index = 0;
        let mut in_num = false;
        let mut num_ind = 0;

        for c in line.chars() {
            if c.is_numeric() {
                // Number
                if !in_num {
                    num_ind = index;
                }
                in_num = true;
            } else if c == '.' {
                // nothing
                if in_num {
                    numbers.push(NumSym{ line: line_no, col: num_ind, len: index - num_ind});
                }
                in_num = false;
            } else {
                // Symbol
                if in_num {
                    numbers.push(NumSym{ line: line_no, col: num_ind, len: index - num_ind});
                }
                symbols.push(NumSym{ line: line_no, col: index, len: 1});

                in_num = false;
            }
            index = index + 1;
        }
        if in_num {
            numbers.push(NumSym{ line: line_no, col: num_ind, len: index - num_ind});
        }
        line_no = line_no + 1;
    }

    Some(NumbersAndSymbols { numbers: numbers, symbols: symbols })
}

fn is_number_a_part(number: NumSym, symbols: &Vec<NumSym>) -> bool {
    for numsym in symbols {
        if (number.line == numsym.line) && ((number.col == numsym.col + 1) || (number.col + number.len == numsym.col)) {
            return true;
        } else if ((number.line == numsym.line + 1) || (number.line + 1 == numsym.line)) &&
                  ((numsym.col + 1 >= number.col) && (numsym.col <= number.col + number.len)){
            return true;
        }
    }
    false
}

fn parse_number(number: NumSym, data: &Vec<String>) -> Option<u32> {
    data[number.line][number.col..number.col+number.len].parse::<u32>().ok()
}

pub fn get_part_numbers(data: &Vec<String>) -> Option<Vec<u32>> {

    let mut result: Vec<u32> = Vec::new();

    let pso = get_numbers_and_symbols(data);

    if pso.is_none() {
        return None;
    }
    let ps = pso.unwrap();

    for num in ps.numbers {
        if is_number_a_part(num, &ps.symbols) {
            let number = parse_number(num, data);
            if number.is_some() {
                result.push(number.unwrap());
            }
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn fill_input() -> Vec<String> {
        let mut data = Vec::new();
        
        data.push("467..114..".to_string());
        data.push("...*......".to_string());
        data.push("..35..633.".to_string());
        data.push("......#...".to_string());
        data.push("617*......".to_string());
        data.push(".....+..58".to_string());
        data.push("..592.....".to_string());
        data.push("......755.".to_string());
        data.push("...$.*....".to_string());
        data.push(".664.598..".to_string());

        data
    }

    #[test]
    fn test_get_numbers_and_symbols() {
        let data = fill_input();

        let numsyms = get_numbers_and_symbols(&data);

        assert_eq!(numsyms.is_none(), false);

        assert_eq!(numsyms.as_ref().unwrap().numbers.len(), 10);

        assert_eq!(numsyms.as_ref().unwrap().symbols.len(), 6);
    }

    #[test]
    fn test_is_number_a_part() {
        let mut numbers = Vec::new();

        numbers.push(NumSym{ line: 0, col: 0, len: 3});
        numbers.push(NumSym{ line: 0, col: 5, len: 3});

        numbers.push(NumSym{ line: 2, col: 2, len: 2});
        numbers.push(NumSym{ line: 2, col: 6, len: 3});

        let mut symbols = Vec::new();

        symbols.push(NumSym{ line: 1, col: 3, len: 1});
        symbols.push(NumSym{ line: 3, col: 6, len: 1});

        assert_eq!(is_number_a_part(numbers[0], &symbols), true);
        assert_eq!(is_number_a_part(numbers[1], &symbols), false);
        assert_eq!(is_number_a_part(numbers[2], &symbols), true);
        assert_eq!(is_number_a_part(numbers[3], &symbols), true);

    }

    #[test]
    fn test_parse_number() {
        let data = fill_input();

        let n1 = NumSym{ line: 0, col: 0, len: 3};

        let n1_num = parse_number(n1, &data);

        assert_eq!(n1_num.is_none(), false);
        assert_eq!(n1_num.unwrap(), 467);

        let n2 = NumSym{ line: 2, col: 2, len: 2};

        let n2_num = parse_number(n2, &data);

        assert_eq!(n2_num.is_none(), false);
        assert_eq!(n2_num.unwrap(), 35);
    }

    #[test]
    fn test_get_part_numbers() {
        let data = fill_input();

        let part_numbers = get_part_numbers(&data);

        assert_eq!(part_numbers.is_none(), false);
        assert_eq!(part_numbers.as_ref().unwrap().len(), 8);
        assert_eq!(part_numbers.as_ref().unwrap()[0], 467);
        assert_eq!(part_numbers.as_ref().unwrap()[1], 35);
        assert_eq!(part_numbers.as_ref().unwrap()[2], 633);
        assert_eq!(part_numbers.as_ref().unwrap()[3], 617);
        assert_eq!(part_numbers.as_ref().unwrap()[4], 592);
        assert_eq!(part_numbers.as_ref().unwrap()[5], 755);
        assert_eq!(part_numbers.as_ref().unwrap()[6], 664);
        assert_eq!(part_numbers.as_ref().unwrap()[7], 598);
    }

}