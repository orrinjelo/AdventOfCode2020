use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

#[derive(Clone, Debug, PartialEq)]
enum OperatorState {
    None,
    Add(u128),
    Multiply(u128),
}

#[derive(Clone, Debug, PartialEq)]
enum MathElement {
    OperatorAdd(u128),
    OperatorMul,
    Number(u128),
    // ParenOpen,
    // ParenClose,
}

fn parse_maths(input: &String) -> u128 {
    let mut i = 0;
    parse_maths_indexed(input, &mut i)
}

fn parse_advanced_maths(input: &String) -> u128 {
    let mut i = 0;
    parse_advanced_maths_indexed(input, &mut i)
}

fn parse_maths_indexed(input: &String, i: &mut u128) -> u128 {
    let mut operator_stack: OperatorState = OperatorState::None;
    let mut number_stack: u128 = 0;

    while *i != input.len() as u128 {
        // debug!("i: {}, op: {:?}, n: {}", i, operator_stack, number_stack);
        match input.chars().nth(*i as usize) {
            Some(' ') => {},
            Some('+') => {
                operator_stack = OperatorState::Add(number_stack);
            },
            Some('*') => {
                operator_stack = OperatorState::Multiply(number_stack);
            },
            Some(')') => {
                return number_stack;
            },
            Some('(') => {
                *i += 1;
                let num = parse_maths_indexed(input, i);

                match operator_stack {
                    OperatorState::None => {
                        number_stack = num;
                    },
                    OperatorState::Add(n) => {
                        number_stack = n + num;
                        operator_stack = OperatorState::None;
                    },
                    OperatorState::Multiply(n) => {
                        number_stack = n * num;
                        operator_stack = OperatorState::None;
                    }
                }
            },
            Some(x) => {
                let num: u128 = x.to_digit(10).unwrap() as u128;
                match operator_stack {
                    OperatorState::None => {
                        number_stack = num;
                    },
                    OperatorState::Add(n) => {
                        number_stack = n + num;
                        operator_stack = OperatorState::None;
                    },
                    OperatorState::Multiply(n) => {
                        number_stack = n * num;
                        operator_stack = OperatorState::None;
                    }
                }
            },
            None => error!("Couldn't get character."),
        }
        *i += 1;
    }
    number_stack
}

fn parse_advanced_maths_indexed(input: &String, i: &mut u128) -> u128 {
    let mut stack: Vec<MathElement> = Vec::new();

    while *i != input.len() as u128 {
        match input.chars().nth(*i as usize) {
            Some(' ') => {},
            Some('+') => {
                if let MathElement::Number(value) = stack.pop().unwrap() {
                    stack.push(MathElement::OperatorAdd(value));
                }
            },
            Some('*') => {
                stack.push(MathElement::OperatorMul);
            },
            Some('(') => {
                *i += 1;
                stack.push(
                    MathElement::Number(
                        parse_advanced_maths_indexed(input, i)
                    )
                );
            },
            Some(')') => {
                break;
            },
            Some(x) => {
                let mut hold_on_to: u128 = x.to_digit(10).unwrap() as u128;
                'AddLoop: loop {
                    match stack.last().cloned() {
                        Some(element) => match element {
                            MathElement::OperatorAdd(y) => {
                                stack.pop();
                                debug!("hold_on_to: {}; y: {}", hold_on_to, y);
                                hold_on_to += y;
                                debug!("hold_on_to: {}; y: {}", hold_on_to, y);
                            },
                            _ => { 
                                hold_on_to = x.to_digit(10).unwrap() as u128;
                                break 'AddLoop;
                            },
                        },
                        None => {
                            break 'AddLoop;
                        },
                    }
                }
                debug!("hold_on_to: {}", hold_on_to);
                stack.push(MathElement::Number(hold_on_to));
            },
            None => { error!("Invalid character."); }
        }
        *i += 1;
        debug!("Stack: {:?}", stack);

    }

    debug!("Stack: {:?}", stack);

    let mut value: u128 = 1;
    let mut op: OperatorState = OperatorState::None;
    while stack.len() != 0 {
        match stack.last().cloned() {
            Some(element) => {
                match element {
                    MathElement::Number(x) => {
                        match op {
                            OperatorState::Multiply(y) => {
                                value = x * y;
                                op = OperatorState::None;
                            },
                            OperatorState::None => {
                                value = x;
                            }
                            _ => { error!("Got unexpected operator state: {:?}", op); }
                        }
                    },
                    MathElement::OperatorMul => { 
                        op = OperatorState::Multiply(value); 
                    },
                    e => { error!("Got unexpected MathElement: {:?}", e); }
                }
            },
            None => { break; }
        }
        stack.pop();
    }

    value
}


/// Problem #18, part 1
pub fn problem_181(input: Vec<String>) -> RetType {
    RetType::U128(input.iter().map(|x| parse_maths(x)).sum())
}

/// Problem #18, part 2
pub fn problem_182(input: Vec<String>) -> RetType {
    RetType::U128(input.iter().map(|x| parse_advanced_maths(x)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        match env_logger::try_init() {
            Ok(_) => {
                info!("Initializing logging...");
            },
            Err(_) => {

            }
        }
    }

    #[test]
    fn test_maths() {
        init();

        assert_eq!(parse_maths(&"3 + 2".to_string()), 5);
        assert_eq!(parse_maths(&"(3 + 2)".to_string()), 5);
        assert_eq!(parse_maths(&"(3 * 2)".to_string()), 6);
        assert_eq!(parse_maths(&"6 * (3 + 2)".to_string()), 30);
        assert_eq!(parse_maths(&"(3 + 3) * (3 + 2) + 2 * 2".to_string()), 64);
    }

    #[test]
    fn test_advanced_maths() {
        init();

        // assert_eq!(parse_advanced_maths(&"3 + 2".to_string()), 5);
        // assert_eq!(parse_advanced_maths(&"(3 + 2)".to_string()), 5);
        // assert_eq!(parse_advanced_maths(&"(3 * 2)".to_string()), 6);
        // assert_eq!(parse_advanced_maths(&"6 * (3 + 2)".to_string()), 30);
        // assert_eq!(parse_advanced_maths(&"(3 + 3) * (3 + 2) + 2 * 2".to_string()), 84);
        // assert_eq!(parse_advanced_maths(&"1 + (2 * 3) + (4 * (5 + 6))".to_string()), 51);
    }
}