pub mod problems;
pub mod aoclib;

// use std::env;
use log::{info, warn}; // trace, debug, info, warn, error
use env_logger;
use std::time::Instant;
use rustop::opts;

fn execute_problem(num: i32, filename: String, part1: fn(String) -> u32, part2: fn(String) -> u32) {
    let start = Instant::now();
    let result_1 = part1(filename.clone());
    let then_elapsed = start.elapsed();
    let then = Instant::now();
    let result_2 = part2(filename.clone());
    let end_elapsed = then.elapsed();
    info!("Problem {}; Part 1: {} (Runtime: {} μs)", num, result_1, then_elapsed.as_micros());
    info!("Problem {}; Part 2: {} (Runtime: {} μs)", num, result_2, end_elapsed.as_micros());    
}

fn execute_problem_u128(num: i32, filename: String, part1: fn(String) -> u128, part2: fn(String) -> u128) {
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

    let opts = opts! {
        synopsis "Advent of Code 2020";
        param number:Option<i32>, desc:"Problem number to run.";
    };

    let (args, _rest) = opts.parse_or_exit();

    info!("{:?}", args.number);

    info!("==== Advent of Code 2020 ====");

    // Parse args
    if let Some(num) = args.number {
        let filename = format!("aoc2020/inputs/{:02}.txt", num);
        match num {
            // Example problem (problem from last year!)
            0 => execute_problem(num, filename, problems::problem_001, problems::problem_002),
            // Problem 1; Elven Financemancy
            1 => execute_problem(num, filename, problems::problem_011, problems::problem_012),
            // Problem 2; Toboggan Password Problems
            2 => execute_problem(num, filename, problems::problem_021, problems::problem_022),
            // Problem 3; Toboggan Meets Tree
            3 => execute_problem_u128(num, filename, problems::problem_031, problems::problem_032),
            // Problem 4; Dubious Passport Fenangling
            4 => execute_problem(num, filename, problems::problem_041, problems::problem_042),
            // Problem 5; Boarding Pass Bungaloo
            5 => execute_problem(num, filename, problems::problem_051, problems::problem_052),
            _ => warn!("Problem number not available.")
        }
    }
}
