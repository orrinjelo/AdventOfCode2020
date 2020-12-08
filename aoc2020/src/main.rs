mod problems;
mod util;
mod matrix;
mod virtualmachine;

use crate::util::load_file;

// use std::env;
use log::{info, warn}; // trace, debug, info, warn, error
use env_logger;
use std::time::Instant;
use rustop::opts;

macro_rules! ifelse {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {$v} else {$v1}
    };
}

fn execute_problem(num: i32, input: Vec<String>, part1: fn(Vec<String>) -> u32, part2: fn(Vec<String>) -> u32) {
    let start = Instant::now();
    let result_1 = part1(input.clone());
    let then_elapsed = start.elapsed();
    let then = Instant::now();
    let result_2 = part2(input.clone());
    let end_elapsed = then.elapsed();
    info!("Problem {}; Part 1: {} (Runtime: {} μs)", num, result_1, then_elapsed.as_micros());
    info!("Problem {}; Part 2: {} (Runtime: {} μs)", num, result_2, end_elapsed.as_micros());    
}

fn execute_problem_i32(num: i32, input: Vec<String>, part1: fn(Vec<String>) -> i32, part2: fn(Vec<String>) -> i32) {
    let start = Instant::now();
    let result_1 = part1(input.clone());
    let then_elapsed = start.elapsed();
    let then = Instant::now();
    let result_2 = part2(input.clone());
    let end_elapsed = then.elapsed();
    info!("Problem {}; Part 1: {} (Runtime: {} μs)", num, result_1, then_elapsed.as_micros());
    info!("Problem {}; Part 2: {} (Runtime: {} μs)", num, result_2, end_elapsed.as_micros());    
}

fn execute_problem_u128(num: i32, input: Vec<String>, part1: fn(Vec<String>) -> u128, part2: fn(Vec<String>) -> u128) {
    let start = Instant::now();
    let result_1 = part1(input.clone());
    let then_elapsed = start.elapsed();
    let then = Instant::now();
    let result_2 = part2(input.clone());
    let end_elapsed = then.elapsed();
    info!("Problem {}; Part 1: {} (Runtime: {} μs)", num, result_1, then_elapsed.as_micros());
    info!("Problem {}; Part 2: {} (Runtime: {} μs)", num, result_2, end_elapsed.as_micros());    
}

fn main() {
    // Set up logging
    env_logger::init();

    let opts = opts! {
        synopsis "Advent of Code 2020";
        opt input_file:Option<String>, desc: "Custom input file for this problem.";
        param number:Option<i32>, desc:"Problem number to run.";
    };

    let (args, _rest) = opts.parse_or_exit();

    info!("{:?}", args.number);

    info!("==== Advent of Code 2020 ====");

    // Parse args
    if let Some(num) = args.number {
        let filename = ifelse!(args.input_file.is_none(), format!("aoc2020/inputs/{:02}.txt", num).to_string(), args.input_file.unwrap());

        let input = load_file(filename);
        match num {
            // Example problem (problem from last year!)
            0 => execute_problem(num, input, problems::problem00::problem_001, problems::problem00::problem_002),
            // Problem 1; Elven Financemancy
            1 => execute_problem(num, input, problems::problem01::problem_011, problems::problem01::problem_012),
            // Problem 2; Toboggan Password Problems
            2 => execute_problem(num, input, problems::problem02::problem_021, problems::problem02::problem_022),
            // Problem 3; Toboggan Meets Tree
            3 => execute_problem_u128(num, input, problems::problem03::problem_031, problems::problem03::problem_032),
            // Problem 4; Dubious Passport Fenangling
            4 => execute_problem(num, input, problems::problem04::problem_041, problems::problem04::problem_042),
            // Problem 5; Boarding Pass Bungaloo
            5 => execute_problem(num, input, problems::problem05::problem_051, problems::problem05::problem_052),
            // Problem 6; Customs are Dumb
            6 => execute_problem(num, input, problems::problem06::problem_061, problems::problem06::problem_062),
            // Problem 7; Bags are dumb
            7 => execute_problem(num, input, problems::problem07::problem_071, problems::problem07::problem_072),
            // Problem 7; Bags are dumb
            8 => execute_problem_i32(num, input, problems::problem08::problem_081, problems::problem08::problem_082),
            _ => warn!("Problem number not available.")
        }
    }
}
