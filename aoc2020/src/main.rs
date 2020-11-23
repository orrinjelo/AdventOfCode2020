extern crate aoclib;

use std::env;

fn usage() {
    println!("Usage: aoc2020 <problem#>");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("==== Advent of Code ====");

    match args.len() {
        2 => {
            match args[1].parse() {
                // Example problem (problem from last year!)
                Ok(0) => {
                    let filename = String::from("inputs/00.txt");
                    let result_001 = aoclib::problem_001(filename.clone());
                    let result_002 = aoclib::problem_002(filename.clone());
                    println!("Result 0; 1: {}", result_001);
                    println!("Result 0; 2: {}", result_002);            
                },
                Ok(_) => usage(),
                Err(_) => println!("Error has occurred?")
            }
        },
        _ => usage()
    }
}
