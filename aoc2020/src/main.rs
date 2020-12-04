pub mod problems;
pub mod aoclib;

use std::env;
use log::{info, warn, error}; // trace, debug, info, warn, error
use env_logger;
use std::time::Instant;

fn usage() {
    warn!("Usage: aoc2020 <problem#>");
}

fn execute_problem(num: u32, filename: String, part1: fn(String) -> u32, part2: fn(String) -> u32) {
    let start = Instant::now();
    let result_1 = part1(filename.clone());
    let then_elapsed = start.elapsed();
    let then = Instant::now();
    let result_2 = part2(filename.clone());
    let end_elapsed = then.elapsed();
    info!("Problem {}; Part 1: {} (Runtime: {} μs)", num, result_1, then_elapsed.as_micros());
    info!("Problem {}; Part 2: {} (Runtime: {} μs)", num, result_2, end_elapsed.as_micros());    
}

fn execute_problem_u128(num: u32, filename: String, part1: fn(String) -> u128, part2: fn(String) -> u128) {
    let start = Instant::now();
    let result_1 = part1(filename.clone());
    let then_elapsed = start.elapsed();
    let then = Instant::now();
    let result_2 = part2(filename.clone());
    let end_elapsed = then.elapsed();
    info!("Problem {}; Part 1: {} (Runtime: {} μs)", num, result_1, then_elapsed.as_micros());
    info!("Problem {}; Part 2: {} (Runtime: {} μs)", num, result_2, end_elapsed.as_micros());    
}

fn main() {
    // Set up logging
    env_logger::init();

    // Collect arguments
    let args: Vec<String> = env::args().collect();

    info!("==== Advent of Code 2020 ====");

    // Parse args
    match args.len() {
        2 => {
            // TODO: If error, this will panic
            let num = args[1].parse().unwrap();
            let filename = format!("aoc2020/inputs/{:02}.txt", num);
            match args[1].parse() {
                // Example problem (problem from last year!)
                Ok(0) => execute_problem(num, filename, problems::problem_001, problems::problem_002),
                // Problem 1; Elven Financemancy
                Ok(1) => execute_problem(num, filename, problems::problem_011, problems::problem_012),
                // Problem 2; Toboggan Password Problems
                Ok(2) => execute_problem(num, filename, problems::problem_021, problems::problem_022),
                // Problem 3; Toboggan Meets Tree
                Ok(3) => execute_problem_u128(num, filename, problems::problem_031, problems::problem_032),
                // Problem 4; Dubious Passport Fenangling
                Ok(4) => execute_problem(num, filename, problems::problem_041, problems::problem_042),                
                Ok(_) => usage(),
                Err(_) => error!("Error has occurred?")
            }
        },
        _ => usage()
    }
}
