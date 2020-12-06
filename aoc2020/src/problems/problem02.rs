use regex::Regex;
use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

/// Parse a password line
///  1-3 a: abcde
///  1-3 b: cdefg
fn parse_password_line(input_str: String) -> (usize, usize, char, String) {
    let re = Regex::new(r"(\d+)-(\d+)\s(.):\s(.*)").unwrap();
    let cap = re.captures(&input_str).unwrap();

    return (cap[1].parse::<usize>().unwrap(), 
            cap[2].parse::<usize>().unwrap(), 
            cap[3].parse::<char>().unwrap(), 
            cap[4].parse::<String>().unwrap());
}

/// 1-3 a: abcde
/// 1-3 b: cdefg
/// 2-9 c: ccccccccc
/// 
/// In the above example, 2 passwords are valid. The middle password, cdefg, is 
///  not; it contains no instances of b, but needs at least 1. The first and 
///  third passwords are valid: they contain one a or nine c, both within the 
///  limits of their respective policies.
fn is_valid_sled_password_tuple(entry: (usize, usize, char, String)) -> bool {
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

/// Each policy actually describes two positions in the password, where 1 means the 
///  first character, 2 means the second character, and so on. (Be careful; Toboggan 
///  Corporate Policies have no concept of "index zero"!) Exactly one of these 
///  positions must contain the given letter. Other occurrences of the letter 
///  are irrelevant for the purposes of policy enforcement.
/// Given the same example list from above:
///  1-3 a: abcde is valid: position 1 contains a and position 3 does not.
///  1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
///  2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
fn is_valid_toboggan_password_tuple(entry: (usize, usize, char, String)) -> bool {
    let yes_index = entry.0 - 1;
    let no_index  = entry.1 - 1;
    let letter    = entry.2;
    let pass_str  = entry.3.as_bytes();

    if (pass_str[yes_index] as char == letter) ^ (pass_str[no_index] as char == letter) {
        return true;
    }

    return false;
}

/**
 *  Problem #02, part 1
 *  To try to debug the problem, they have created a list (your puzzle 
 *  input) of passwords (according to the corrupted database) and the 
 *  corporate policy when that password was set.
 * How many passwords are valid according to their policies?
 */
pub fn problem_021(input: Vec<String>) -> u32 {
    let mut count: u32 = 0;
    for line in input.into_iter() {
        if is_valid_sled_password_tuple(parse_password_line(line)) {
            count += 1;
        }
    }

    return count;
}

/**
 *  Problem #02, part 2
 *  The shopkeeper suddenly realizes that he just accidentally explained 
 *  the password policy rules from his old job at the sled rental place down 
 *  the street! The Official Toboggan Corporate Policy actually works a little 
 *  differently.
 * How many passwords are valid according to the new interpretation of the policies?
 */
pub fn problem_022(input: Vec<String>) -> u32 {
    let mut count: u32 = 0;
    for line in input.into_iter() {
        if is_valid_toboggan_password_tuple(parse_password_line(line)) {
            count += 1;
        }
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

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
}