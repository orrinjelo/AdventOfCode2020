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
