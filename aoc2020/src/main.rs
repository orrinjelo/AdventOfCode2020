mod problems;
mod util;
mod matrix;
mod virtualmachine;

use crate::util::load_file;

// use std::env;
use log::{info, warn, Level}; // trace, debug, info, warn, error
use env_logger;
use std::io::Write;
use std::time::{Instant, Duration};
use rustop::opts;
use crate::util::RetType;

macro_rules! ifelse {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {$v} else {$v1}
    };
}

fn format_time(ts: Duration) -> String {
    if ts.as_micros() >= 1_000_000 {
        return format!("{} s", (ts.as_millis() as f32)/1000.0);
    } else if ts.as_micros() >= 1000 {
        return format!("{} ms", (ts.as_micros() as f32)/1000.0);
    } 
    return format!("{} μs", ts.as_micros());
}

fn execute_problem(num: i32, input: Vec<String>, part1: fn(Vec<String>) -> RetType, part2: fn(Vec<String>) -> RetType) {
    let start = Instant::now();
    let result_1 = part1(input.clone());
    let then_elapsed = start.elapsed();
    let then = Instant::now();
    let result_2 = part2(input.clone());
    let end_elapsed = then.elapsed();
    info!("Problem {}; Part 1: {} (Runtime: {})", num, result_1, format_time(then_elapsed));
    info!("Problem {}; Part 2: {} (Runtime: {})", num, result_2, format_time(end_elapsed));    
}


fn run_problem(num: i32, input: Vec<String>) {
    match num {
        // Example problem (problem from last year!)
        0 => execute_problem(num, input, problems::problem00::problem_001, problems::problem00::problem_002),
        // Problem 1; Elven Financemancy
        1 => execute_problem(num, input, problems::problem01::problem_011, problems::problem01::problem_012),
        // Problem 2; Toboggan Password Problems
        2 => execute_problem(num, input, problems::problem02::problem_021, problems::problem02::problem_022),
        // Problem 3; Toboggan Meets Tree
        3 => execute_problem(num, input, problems::problem03::problem_031, problems::problem03::problem_032),
        // Problem 4; Dubious Passport Fenangling
        4 => execute_problem(num, input, problems::problem04::problem_041, problems::problem04::problem_042),
        // Problem 5; Boarding Pass Bungaloo
        5 => execute_problem(num, input, problems::problem05::problem_051, problems::problem05::problem_052),
        // Problem 6; Customs are Dumb
        6 => execute_problem(num, input, problems::problem06::problem_061, problems::problem06::problem_062),
        // Problem 7; Bags are dumb
        7 => execute_problem(num, input, problems::problem07::problem_071, problems::problem07::problem_072),
        // Problem 8; Kids are dumb
        8 => execute_problem(num, input, problems::problem08::problem_081, problems::problem08::problem_082),
        // Problem 9; Paperclips are OP
        9 => execute_problem(num, input, problems::problem09::problem_091, problems::problem09::problem_092),
        // Problem 10; Adapters are dumb
        10 => execute_problem(num, input, problems::problem10::problem_101, problems::problem10::problem_102),
        // Problem 11; People are dumb and these ones act like bacteria cultures
        11 => execute_problem(num, input, problems::problem11::problem_111, problems::problem11::problem_112),
        // Problem 12; Ships are dumb
        12 => execute_problem(num, input, problems::problem12::problem_121, problems::problem12::problem_122),
        // Problem 13; Buses are dumb
        13 => execute_problem(num, input, problems::problem13::problem_131, problems::problem13::problem_132),
        // Problem 14; What is this?  I don't even know.
        14 => execute_problem(num, input, problems::problem14::problem_141, problems::problem14::problem_142),
        // Problem 15; Number Memory Game
        15 => execute_problem(num, input, problems::problem15::problem_151, problems::problem15::problem_152),
        // Problem 16; Tickets in Another Language
        16 => execute_problem(num, input, problems::problem16::problem_161, problems::problem16::problem_162),
        // Problem 17; Game of Life 3D...I mean, 4D
        17 => execute_problem(num, input, problems::problem17::problem_171, problems::problem17::problem_172),
        // Problem 18; Math is math
        18 => execute_problem(num, input, problems::problem18::problem_181, problems::problem18::problem_182),
        _ => warn!("Problem number not available.")
    }
}

fn main() {
    // Set up logging
    env_logger::builder()
        .format(|buf, record| {
            let mut style = buf.style();

            let color = match record.level() {
                Level::Trace => env_logger::fmt::Color::Magenta,
                Level::Debug => env_logger::fmt::Color::Cyan,
                Level::Info  => env_logger::fmt::Color::Green,
                Level::Warn  => env_logger::fmt::Color::Yellow,
                Level::Error => env_logger::fmt::Color::Red,
            };

            style.set_color(color);
            writeln!(buf, "{}: {}", style.value(record.level()), record.args())
        })
        .init();

    let opts = opts! {
        synopsis "Advent of Code 2020";
        opt run_all:bool, desc: "Run all problems.";
        opt input_file:Option<String>, desc: "Custom input file for this problem.";
        param number:Option<i32>, desc:"Problem number to run.";
    };

    let (args, _rest) = opts.parse_or_exit();

    info!("{:?}", args.number);

    info!("==== Advent of Code 2020 ====");

    // Parse args
    if args.run_all {
        for i in 1..18 {
            let filename = format!("aoc2020/inputs/{:02}.txt", i).to_string();
            let input = load_file(filename);
            run_problem(i, input);
            info!("=========================");
        }
    } else {
        if let Some(num) = args.number {
            let filename = ifelse!(args.input_file.is_none(), format!("aoc2020/inputs/{:02}.txt", num).to_string(), args.input_file.unwrap());
            let input = load_file(filename);
            run_problem(num, input);
        }
    }
}
