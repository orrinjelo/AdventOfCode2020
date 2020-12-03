use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use regex::Regex;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

#[allow(unused_macros)]
macro_rules! ifelse {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {$v} else {$v1}
    };
}

/**
 * @brief Utility function to read lines from a file
 * @details Opens and reads a file, returns a vector of strings 
 *  wrapped in a Result
 * 
 * @param filename - String filename path
 * @return Result of a Vector of Strings
 */
#[allow(dead_code)]
pub fn lines_from_file(filename: String) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

/**
 * @brief Naive approach to calculating fuel
 * @details Divide by 3, subtract 2
 * 
 * @param module_mass - initial mass to calculate fuel for
 * @return Fuel necessary for the module
 */
#[allow(dead_code)]
pub fn calculate_fuel_naive(module_mass: u32) -> u32 {
    return (module_mass / 3) - 2;
}

/**
 * @brief Correct way of calculating fuel for a module
 * @details Divide by 3, subtract 2, then calculate the fuel necessary for
 *  the mass of that fuel, recursively.
 * 
 * @param module_mass - initial mass to calculate fuel for
 * @return Fuel necessary for the module
 */
#[allow(dead_code)]
pub fn calculate_fuel(module_mass: u32) -> u32 {
    if module_mass < 9 {
        return 0;
    }
    let mut res = (module_mass / 3) - 2;
    res += calculate_fuel(res);

    return res;
}

#[allow(dead_code)]
pub fn naive_find_sum_equal_to(input_vector: Vec<u32>, target_value: u32) -> Result<(u32, u32), &'static str> {
    for (i, x) in input_vector.iter().enumerate() {
        for (j, y) in input_vector.iter().enumerate() {
            if i == j {
                continue;
            }
            if x+y == target_value {
                return Ok((*x, *y));
            }
        }
    }
    return Err("No matching pair found.");
}

#[allow(dead_code)]
pub fn find_sum_equal_to(input_vector: Vec<u32>, num_combo: usize, target_value: u32) -> Result<u32, &'static str> {
    let combos = input_vector.iter().combinations(num_combo);
    for entry in combos {
        let entry_it = entry.clone();
        if entry_it.into_iter().sum::<u32>() == target_value {
            let mut prod = 1;
            for x in entry {
                debug!("Entry: {}", x);
                prod *= x;
            }
            return Ok(prod);
        }
    }
    return Err("No matching pair found.");
}

#[allow(dead_code)]
/**
 * @brief Parse a password line
 * @details 1-3 a: abcde
            1-3 b: cdefg
 */
pub fn parse_password_line(input_str: String) -> (usize, usize, char, String) {
    let re = Regex::new(r"(\d+)-(\d+)\s(.):\s(.*)").unwrap();
    let cap = re.captures(&input_str).unwrap();

    return (cap[1].parse::<usize>().unwrap(), 
            cap[2].parse::<usize>().unwrap(), 
            cap[3].parse::<char>().unwrap(), 
            cap[4].parse::<String>().unwrap());
}

#[allow(dead_code)]
/**
 * 1-3 a: abcde
 * 1-3 b: cdefg
 * 2-9 c: ccccccccc
 * 
 * In the above example, 2 passwords are valid. The middle password, cdefg, is 
 *  not; it contains no instances of b, but needs at least 1. The first and 
 *  third passwords are valid: they contain one a or nine c, both within the 
 *  limits of their respective policies.
 */
pub fn is_valid_sled_password_tuple(entry: (usize, usize, char, String)) -> bool {
    let min_count = entry.0;
    let max_count = entry.1;
    let letter    = entry.2;
    let pass_str  = entry.3;

    let num_matches = pass_str.matches(letter).count();
    if num_matches >= min_count && num_matches <= max_count {
        return true;
    } 
    return false;
}

#[allow(dead_code)]
/**
 * Each policy actually describes two positions in the password, where 1 means the 
 *  first character, 2 means the second character, and so on. (Be careful; Toboggan 
 *  Corporate Policies have no concept of "index zero"!) Exactly one of these 
 *  positions must contain the given letter. Other occurrences of the letter 
 *  are irrelevant for the purposes of policy enforcement.
 *
 * Given the same example list from above:
 *
 *  1-3 a: abcde is valid: position 1 contains a and position 3 does not.
 *  1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
 *  2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
 */
pub fn is_valid_toboggan_password_tuple(entry: (usize, usize, char, String)) -> bool {
    let yes_index = entry.0 - 1;
    let no_index  = entry.1 - 1;
    let letter    = entry.2;
    let pass_str  = entry.3.as_bytes();

    if (pass_str[yes_index] as char == letter) ^ (pass_str[no_index] as char == letter) {
        return true;
    }

    return false;
}

#[allow(dead_code)]
/**
 *  ArboralLandscape
 *  Due to the local geology, trees in this area only grow on exact integer coordinates 
 *   in a grid. You make a map (your puzzle input) of the open squares (.) and trees (#) 
 *   you can see.
 *  These aren't the only trees, though; due to something you read about once involving 
 *   arboreal genetics and biome stability, the same pattern repeats to the right many times.
 */
pub struct ArboralLandscape {
   tree_map: Vec<Vec<u8>>, 
}

impl ArboralLandscape {
    pub fn new(string_map: Vec<String>) -> ArboralLandscape {
        let rows = string_map.len();
        let cols = string_map[0].len();

        trace!("rows: {}", rows);
        trace!("cols: {}", cols);

        let mut tm = vec![vec![0; cols]; rows];

        trace!("tm rows: {}", tm.len());
        trace!("tm cols: {}", tm[0].len());

        for row_num in 0..rows {
            for col_num in 0..cols {
                if string_map[row_num].chars().nth(col_num).unwrap() == '#' {
                    tm[row_num][col_num] = 1;
                }
            }
        }
        ArboralLandscape {
            tree_map: tm,
        }
    }

    pub fn print_n(&self, repeat_n: usize) {
        for row in &self.tree_map {
            for c in 0..row.len()*repeat_n {
                print!("{}", ifelse!(row[c%row.len()]==1u8, '#', '.'));
            }
            println!();
        }
    }

    pub fn print(&self) {
        self.print_n(1);
    }

    pub fn traverse(&self, down: u8, right: u8) -> u128 {
        let mut tree_count: u128 = 0;
        let mut position = (0, 0);
        let row_len = self.tree_map[0].len();
        while position.0 < self.tree_map.len() {
            tree_count += self.tree_map[position.0][position.1 % row_len] as u128;
            position = (position.0 + down as usize, position.1 + right as usize);
        }
        return tree_count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::{error};

    fn init() {
        match env_logger::try_init() {
            Ok(_) => {
                info!("Initializing logging...");
            },
            Err(_) => {

            }
        }
    }

    #[test]
    /**
    For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
    For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
    For a mass of 1969, the fuel required is 654.
    For a mass of 100756, the fuel required is 33583.
    */
    fn test_calculate_fuel_naive() {
        init();
        assert_eq!(calculate_fuel_naive(12),         2);
        assert_eq!(calculate_fuel_naive(14),         2);
        assert_eq!(calculate_fuel_naive(1969),     654);
        assert_eq!(calculate_fuel_naive(100756), 33583);
    }

    #[test]
    /**
    A module of mass 14 requires 2 fuel. This fuel requires no further fuel (2 divided by 3 and rounded 
     down is 0, which would call for a negative fuel), so the total fuel required is still just 2.
    At first, a module of mass 1969 requires 654 fuel. Then, this fuel requires 216 more fuel (654 / 3 - 2). 
     216 then requires 70 more fuel, which requires 21 fuel, which requires 5 fuel, which requires no 
     further fuel. So, the total fuel required for a module of mass 1969 is 654 + 216 + 70 + 21 + 5 = 966.
    The fuel required by a module of mass 100756 and its fuel is: 33583 + 11192 + 3728 + 1240 + 411 + 135 + 
     43 + 12 + 2 = 50346.
    */
    fn test_calculate_fuel() {
        init();
        assert_eq!(calculate_fuel(14),         2);
        assert_eq!(calculate_fuel(1969),     966);
        assert_eq!(calculate_fuel(100756), 50346);
    }

    #[test]
    /*
    For example, suppose your expense report contained the following:

    1721
    979
    366
    299
    675
    1456

    In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces 
     1721 * 299 = 514579, so the correct answer is 514579.
    */
    fn test_naive_find_sum_equal_to() {
        init();
        let input_vec = vec![1721, 979, 366, 299, 675, 1456];
        let result = naive_find_sum_equal_to(input_vec, 2020);
        match result {
            Ok((1721, 299)) => {
                assert!(true);
            },
            Ok(_) => {
                error!("Wrong result: {} {}", result.unwrap().0, result.unwrap().1);
                assert!(false);
            },
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    /**
    Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. 
     Multiplying them together produces the answer, 241861950.
    */
    fn test_find_sum_equal_to() {
        init();
        let input_vec = vec![1721, 979, 366, 299, 675, 1456];
        let result = find_sum_equal_to(input_vec, 3, 2020);
        match result {
            Ok(241861950) => {
                assert!(true);
            },
            Ok(_) => {
                error!("Wrong result: {}", result.unwrap());
                assert!(false);
            },
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_parse_password_line() {
        init();
        let string_one = String::from("1-3 a: abcde");
        let string_two = String::from("1-3 b: cdefg");
        let string_three = String::from("2-9 c: ccccccccc");
        let string_four = String::from("12-19 d: goodjorb");

        let res_one = parse_password_line(string_one);
        let res_two = parse_password_line(string_two);
        let res_three = parse_password_line(string_three);
        let res_four = parse_password_line(string_four);

        assert_eq!(res_one, (1, 3, 'a', String::from("abcde")));
        assert_eq!(res_two, (1, 3, 'b', String::from("cdefg")));
        assert_eq!(res_three, (2, 9, 'c', String::from("ccccccccc")));
        assert_eq!(res_four, (12, 19, 'd', String::from("goodjorb")));
    }

    #[test]
    fn test_is_valid_sled_password_tuple() {
        init();
        assert!(is_valid_sled_password_tuple((1, 3, 'a', String::from("abcde"))));
        assert!(!is_valid_sled_password_tuple((1, 3, 'b', String::from("cdefg"))));
        assert!(is_valid_sled_password_tuple((2, 9, 'c', String::from("ccccccccc"))));
    }

    #[test]
    fn test_is_valid_toboggan_password_tuple() {
        init();
        assert!(is_valid_toboggan_password_tuple((1, 3, 'a', String::from("abcde"))));
        assert!(!is_valid_toboggan_password_tuple((1, 3, 'b', String::from("cdefg"))));
        assert!(!is_valid_toboggan_password_tuple((2, 9, 'c', String::from("ccccccccc"))));
        assert!(is_valid_toboggan_password_tuple((15, 16, 'o', String::from("abcdefghijklmnop"))));
        assert!(is_valid_toboggan_password_tuple((11, 12, 'q', String::from("qqqkqkqqqqzqqq"))));
    }

    #[test]
    fn test_new_and_print_arboral_landscape() {
        init();
        let string_map = vec![
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string()
        ];
        let arboral_landscape = ArboralLandscape::new(string_map);
        arboral_landscape.print_n(6);
    }

    #[test]
    /**
        Right 1, down 1.
        Right 3, down 1.
        Right 5, down 1.
        Right 7, down 1.
        Right 1, down 2.

        In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s) respectively; 
        multiplied together, these produce the answer 336.
     */
    fn test_traverse_arboral_landscape() {
        init();
        let string_map = vec![
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string()
        ];
        let arboral_landscape = ArboralLandscape::new(string_map);
        assert_eq!(arboral_landscape.traverse(1,1), 2);
        assert_eq!(arboral_landscape.traverse(1,3), 7);
        assert_eq!(arboral_landscape.traverse(1,5), 3);
        assert_eq!(arboral_landscape.traverse(1,7), 4);
        assert_eq!(arboral_landscape.traverse(2,1), 2);
    }

    #[test]
    fn test_traverse_arboral_landscape_part_2() {
        init();
        let string_map = vec![
            "....".to_string(),
            "....".to_string(),
            ".#..".to_string()
        ];
        let arboral_landscape = ArboralLandscape::new(string_map);
        assert_eq!(arboral_landscape.traverse(2,1), 1);
    }
}
