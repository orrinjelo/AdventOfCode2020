use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use log::{debug}; // trace, debug, info, warn, error

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


#[cfg(test)]
mod tests {
    use super::*;
    use log::{error};

    #[test]
    /**
    For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
    For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
    For a mass of 1969, the fuel required is 654.
    For a mass of 100756, the fuel required is 33583.
    */
    fn test_calculate_fuel_naive() {
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
