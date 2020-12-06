use itertools::Itertools;
use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

fn naive_find_sum_equal_to(input_vector: Vec<u32>, target_value: u32) -> Result<(u32, u32), &'static str> {
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

fn find_sum_equal_to(input_vector: Vec<u32>, num_combo: usize, target_value: u32) -> Result<u32, &'static str> {
    let combos = input_vector.iter().combinations(num_combo);
    for entry in combos {
        let entry_it = entry.clone();
        if entry_it.into_iter().sum::<u32>() == target_value {
            let mut prod = 1;
            for x in entry {
                trace!("Entry: {}", x);
                prod *= x;
            }
            return Ok(prod);
        }
    }
    return Err("No matching pair found.");
}

/**
 *  Problem #01, part 1
 *  Before you leave, the Elves in accounting just need you to fix 
 *  your expense report (your puzzle input); apparently, something isn't 
 *  quite adding up. 
 * Specifically, they need you to find the two entries that sum to 2020 and 
 *  then multiply those two numbers together.
 */
pub fn problem_011(input: Vec<String>) -> u32 {
    let parsed_input: Vec<u32> = input
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
    ;
    let entry = naive_find_sum_equal_to(parsed_input, 2020).unwrap();
    let result = entry.0 * entry.1;
    return result;
}

/**
 *  Problem #01, part 2
 *  The Elves in accounting are thankful for your help; one of them 
 *  even offers you a starfish coin they had left over from a past vacation. 
 *  They offer you a second one if you can find three numbers in your 
 *  expense report that meet the same criteria.
 */
pub fn problem_012(input: Vec<String>) -> u32 {
    let parsed_input: Vec<u32> = input
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
    ;
    return find_sum_equal_to(parsed_input, 3, 2020).unwrap();
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
    /// For example, suppose your expense report contained the following:
    ///     
    /// 1721
    /// 979
    /// 366
    /// 299
    /// 675
    /// 1456
    ///
    /// In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces 
    ///  1721 * 299 = 514579, so the correct answer is 514579.
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
    /// Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. 
    ///  Multiplying them together produces the answer, 241861950.
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

}