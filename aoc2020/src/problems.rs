use crate::aoclib;
use log::{debug}; // trace, debug, info, warn, error

/**
 *  Problem #00, part 1
 *  Fuel required to launch a given module is based 
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
 *  Problem #00, part 2
 *  During the second Go / No Go poll, the Elf in charge 
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
 *  Problem #01, part 1
 *  Before you leave, the Elves in accounting just need you to fix 
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
 *  Problem #01, part 2
 *  The Elves in accounting are thankful for your help; one of them 
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
 *  Problem #02, part 1
 *  To try to debug the problem, they have created a list (your puzzle 
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
 *  Problem #02, part 2
 *  The shopkeeper suddenly realizes that he just accidentally explained 
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

/**
 *  Problem #03, part 1
 *  With the toboggan login problems resolved, you set off toward the 
 *  airport. While travel by toboggan might be easy, it's certainly not safe: 
 *  there's very minimal steering and the area is covered in trees. You'll 
 *  need to see which angles will take you near the fewest trees.
 * Due to the local geology, trees in this area only grow on exact integer 
 *  coordinates in a grid.
 */
pub fn problem_031(input_file: String) -> u128 {
    let input: Vec<String> = aoclib::lines_from_file(input_file)
        .expect("Could not read from file");

    let mut arboral_landscape = aoclib::ArboralLandscape::new(input);
    return arboral_landscape.traverse(1,3);
}

/**
 *  Problem #03, part 2
 *  Time to check the rest of the slopes - you need to minimize
 *  the probability of a sudden arboreal stop, after all.
 * Determine the number of trees you would encounter if, for each of the 
 *  following slopes, you start at the top-left corner and traverse the map 
 *  all the way to the bottom.
 * What do you get if you multiply together the number of trees encountered 
 *  on each of the listed slopes?
 */
pub fn problem_032(input_file: String) -> u128 {
    let input: Vec<String> = aoclib::lines_from_file(input_file)
        .expect("Could not read from file");

    let mut arboral_landscape = aoclib::ArboralLandscape::new(input);
    let slope_1_1 = arboral_landscape.traverse(1,1);
    debug!("slope_1_1: {}", slope_1_1);
    let slope_1_3 = arboral_landscape.traverse(1,3);
    debug!("slope_1_3: {}", slope_1_3);
    let slope_1_5 = arboral_landscape.traverse(1,5);
    debug!("slope_1_5: {}", slope_1_5);
    let slope_1_7 = arboral_landscape.traverse(1,7);
    debug!("slope_1_7: {}", slope_1_7);
    let slope_2_1 = arboral_landscape.traverse(2,1);
    debug!("slope_2_1: {}", slope_2_1);
    // arboral_landscape.print_n(5);
    return slope_1_1 * slope_1_3 * slope_1_5 * slope_1_7 * slope_2_1;
}

/**
 *  Problem #04, part 1
 *  You arrive at the airport only to realize that you grabbed your 
 *  North Pole Credentials instead of your passport. While these documents 
 *  are extremely similar, North Pole Credentials aren't issued by a country 
 *  and therefore aren't actually valid documentation for travel in most of 
 *  the world.
 * It seems like you're not the only one having problems, though; a very long 
 *  line has formed for the automatic passport scanners, and the delay could 
 *  upset your travel itinerary.
 * Due to some questionable network security, you realize you might be able 
 *  to solve both of these problems at the same time.
 * The automatic passport scanners are slow because they're having trouble 
 *  detecting which passports have all required fields.
 */
pub fn problem_041(input_file: String) -> u32 {
    let input: Vec<String> = aoclib::lines_from_file(input_file)
        .expect("Could not read from file");

    return aoclib::count_valid_passports(aoclib::semi_questionable_passport_factory(input, false));
}

/**
 *  Problem #04, part 2
 *  The line is moving more quickly now, but you overhear airport 
 *  security talking about how passports with invalid data are getting 
 *  through. Better add some data validation, quick!
 */
pub fn problem_042(input_file: String) -> u32 {
    let input: Vec<String> = aoclib::lines_from_file(input_file)
        .expect("Could not read from file");

    return aoclib::count_valid_passports(aoclib::semi_questionable_passport_factory(input, true));
}
