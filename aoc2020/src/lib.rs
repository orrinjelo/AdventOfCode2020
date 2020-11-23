use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

// TODO: Separate this library into a utility library and a problem library

/**
 * @brief Naive approach to calculating fuel
 * @details Divide by 3, subtract 2
 * 
 * @param module_mass - initial mass to calculate fuel for
 * @return Fuel necessary for the module
 */
fn calculate_fuel_naive(module_mass: u32) -> u32 {
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
fn calculate_fuel(module_mass: u32) -> u32 {
    if module_mass < 9 {
        return 0;
    }
    let mut res = (module_mass / 3) - 2;
    res += calculate_fuel(res);

    return res;
}

/**
 * @brief Utility function to read lines from a file
 * @details Opens and reads a file, returns a vector of strings 
 *  wrapped in a Result
 * 
 * @param filename - String filename path
 * @return Result of a Vector of Strings
 */
fn lines_from_file(filename: String) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

/**
 * @brief Problem #00, part 1
 * @details Fuel required to launch a given module is based 
 *  on its mass. Specifically, to find the fuel required for a 
 *  module, take its mass, divide by three, round down, and 
 *  subtract 2.
 * The Fuel Counter-Upper needs to know the total fuel 
 *  requirement. To find it, individually calculate the fuel 
 *  needed for the mass of each module (your puzzle input), 
 *  then add together all the fuel values.
 * What is the sum of the fuel requirements for all of the 
 *  modules on your spacecraft?
 * 
 * @param input_file - filename path to the 
 * @return Returns the numerical result (u32); amount of fuel
 */
pub fn problem_001(input_file: String) -> u32 {
    return lines_from_file(input_file)
        .expect("Could not read from file")
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .map(|x| calculate_fuel_naive(x))
        .sum()
    ;
}

/**
 * @brief Problem #00, part 2
 * @details During the second Go / No Go poll, the Elf in charge 
 *  of the Rocket Equation Double-Checker stops the launch sequence. 
 *  Apparently, you forgot to include additional fuel for the fuel 
 *  you just added. 
 * Fuel itself requires fuel just like a module - take its mass, 
 *  divide by three, round down, and subtract 2. However, that fuel 
 *  also requires fuel, and that fuel requires fuel, and so on. Any 
 *  mass that would require negative fuel should instead be treated 
 *  as if it requires zero fuel; the remaining mass, if any, is 
 *  instead handled by wishing really hard, which has no mass and is 
 *  outside the scope of this calculation.
 * So, for each module mass, calculate its fuel and add it to the 
 *  total. Then, treat the fuel amount you just calculated as the 
 *  input mass and repeat the process, continuing until a fuel 
 *  requirement is zero or negative.
 * 
 * @param input_file - filename path to the 
 * @return Returns the numerical result (u32); amount of fuel
 */
pub fn problem_002(input_file: String) -> u32 {
    return lines_from_file(input_file)
        .expect("Could not read from file")
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .map(|x| calculate_fuel(x))
        .sum()
    ;
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
