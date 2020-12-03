pub mod problems;
pub mod aoclib;

use std::env;
use log::{info, warn, error}; // trace, debug, info, warn, error
use env_logger;
use std::time::Instant;

fn usage() {
    warn!("Usage: aoc2020 <problem#>");
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
            match args[1].parse() {
                // Example problem (problem from last year!)
                Ok(0) => {
                    let filename = String::from("aoc2020/inputs/00.txt");
                    let start = Instant::now();
                    let result_001 = problems::problem_001(filename.clone());
                    let then_elapsed = start.elapsed();
                    let then = Instant::now();
                    let result_002 = problems::problem_002(filename.clone());
                    let end_elapsed = then.elapsed();
                    info!("Problem 0; Part 1: {} (Runtime: {} μs)", result_001, then_elapsed.as_micros());
                    info!("Problem 0; Part 2: {} (Runtime: {} μs)", result_002, end_elapsed.as_micros());        
                },
                // Problem 1; Elven Financemancy
                Ok(1) => {
                    let filename = String::from("aoc2020/inputs/01.txt");
                    let start = Instant::now();
                    let result_011 = problems::problem_011(filename.clone());
                    let then_elapsed = start.elapsed();
                    let then = Instant::now();
                    let result_012 = problems::problem_012(filename.clone());
                    let end_elapsed = then.elapsed();
                    info!("Problem 1; Part 1: {} (Runtime: {} μs)", result_011, then_elapsed.as_micros());
                    info!("Problem 1; Part 2: {} (Runtime: {} ms)", result_012, end_elapsed.as_millis());
                },
                // Problem 2; Toboggan Password Problems
                Ok(2) => {
                    let filename = String::from("aoc2020/inputs/02.txt");
                    let start = Instant::now();
                    let result_021 = problems::problem_021(filename.clone());
                    let then_elapsed = start.elapsed();
                    let then = Instant::now();
                    let result_022 = problems::problem_022(filename.clone());
                    let end_elapsed = then.elapsed();
                    info!("Problem 2; Part 1: {} (Runtime: {} ms)", result_021, then_elapsed.as_millis());
                    info!("Problem 2; Part 2: {} (Runtime: {} ms)", result_022, end_elapsed.as_millis());
                },
                // Problem 3; Toboggan Meets Tree
                Ok(3) => {
                    let filename = String::from("aoc2020/inputs/03.txt");
                    let start = Instant::now();
                    let result_031 = problems::problem_031(filename.clone());
                    let then_elapsed = start.elapsed();
                    let then = Instant::now();
                    let result_032 = problems::problem_032(filename.clone());
                    let end_elapsed = then.elapsed();
                    info!("Problem 3; Part 1: {} (Runtime: {} ms)", result_031, then_elapsed.as_millis());
                    info!("Problem 3; Part 2: {} (Runtime: {} ms)", result_032, end_elapsed.as_millis());
                },
                Ok(_) => usage(),
                Err(_) => error!("Error has occurred?")
            }
        },
        _ => usage()
    }
}
