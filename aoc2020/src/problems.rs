use crate::aoclib;

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
    return aoclib::lines_from_file(input_file)
        .expect("Could not read from file")
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .map(|x| aoclib::calculate_fuel_naive(x))
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
    return aoclib::lines_from_file(input_file)
        .expect("Could not read from file")
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .map(|x| aoclib::calculate_fuel(x))
        .sum()
    ;
}

/**
 * @brief Problem #01, part 1
 * @details Before you leave, the Elves in accounting just need you to fix 
 *  your expense report (your puzzle input); apparently, something isn't 
 *  quite adding up. 
 * Specifically, they need you to find the two entries that sum to 2020 and 
 *  then multiply those two numbers together.
 */
pub fn problem_011(input_file: String) -> u32 {
    let input: Vec<u32> = aoclib::lines_from_file(input_file)
        .expect("Could not read from file")
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
    ;
    let entry = aoclib::naive_find_sum_equal_to(input, 2020).unwrap();
    let result = entry.0 * entry.1;
    return result;
}

/**
 * @brief Problem #01, part 2
 * @details The Elves in accounting are thankful for your help; one of them 
 *  even offers you a starfish coin they had left over from a past vacation. 
 *  They offer you a second one if you can find three numbers in your 
 *  expense report that meet the same criteria.
 */
pub fn problem_012(input_file: String) -> u32 {
    let input: Vec<u32> = aoclib::lines_from_file(input_file)
        .expect("Could not read from file")
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
    ;
    return aoclib::find_sum_equal_to(input, 3, 2020).unwrap();
}

/**
 * @brief Problem #02, part 1
 * @details To try to debug the problem, they have created a list (your puzzle 
 *  input) of passwords (according to the corrupted database) and the 
 *  corporate policy when that password was set.
 * How many passwords are valid according to their policies?
 */
pub fn problem_021(input_file: String) -> u32 {
    let input: Vec<String> = aoclib::lines_from_file(input_file)
        .expect("Could not read from file");

    let mut count: u32 = 0;
    for line in input.into_iter() {
        if aoclib::is_valid_sled_password_tuple(aoclib::parse_password_line(line)) {
            count += 1;
        }
    }

    return count;
}

/**
 * @brief Problem #02, part 2
 * @details The shopkeeper suddenly realizes that he just accidentally explained 
 *  the password policy rules from his old job at the sled rental place down 
 *  the street! The Official Toboggan Corporate Policy actually works a little 
 *  differently.
 * How many passwords are valid according to the new interpretation of the policies?
 */
pub fn problem_022(input_file: String) -> u32 {
    let input: Vec<String> = aoclib::lines_from_file(input_file)
        .expect("Could not read from file");

    let mut count: u32 = 0;
    for line in input.into_iter() {
        if aoclib::is_valid_toboggan_password_tuple(aoclib::parse_password_line(line)) {
            count += 1;
        }
    }

    return count;
}