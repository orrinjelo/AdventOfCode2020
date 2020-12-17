use std::collections::HashSet;
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

fn build_summation_hash_set(input: Vec<u128>) -> HashSet<u128> {
    let mut output: HashSet<u128> = HashSet::new();
    for i in 0..input.len()-1 {
        for j in i+1..input.len() {
            output.insert(input[i]+input[j]);
        }
    }
    output
}

fn build_sliding_window(input: Vec<u128>, width: usize, offset: usize) -> Vec<u128> {
    input[offset..(offset+width)].to_vec()
}

fn find_sum_subset(input: Vec<u128>, target_idx: usize) -> Vec<u128> {
    let target_val = input[target_idx];

    for offset in 0..target_idx-1 {
        for width in 2..target_idx-offset {
            let contiguous = build_sliding_window(input.clone(), width, offset);
            if contiguous.iter().sum::<u128>() == target_val {
                return contiguous;
            }
        }
    }

    Vec::new()
}

/// Problem #09, part 1
/// The first step of attacking the weakness in the XMAS data is to find the 
///  first number in the list (after the preamble) which is not the sum of two 
///  of the 25 numbers before it. What is the first number that does not have 
///  this property?
pub fn problem_091(input: Vec<String>) -> RetType {
    let parsed_input: Vec<u128> = input
        .into_iter()
        .map(|x| x.parse::<u128>().unwrap())
        .collect()
    ;

    let width: usize = 25;

    for i in 0..parsed_input.len()-width {
        let flag = build_summation_hash_set(
            build_sliding_window(parsed_input.clone(), width, i)
        ).contains(&parsed_input[i+width]);
        if !flag {
            return RetType::U128(parsed_input[i+width]);
        }
    }

    RetType::U128(0u128)
}

/// Problem #09, part 2
/// What is the encryption weakness in your XMAS-encrypted list of numbers?
pub fn problem_092(input: Vec<String>) -> RetType {
    let parsed_input: Vec<u128> = input
        .into_iter()
        .map(|x| x.parse::<u128>().unwrap())
        .collect()
    ;

    let width: usize = 25;

    for i in 0..parsed_input.len()-width {
        let flag = build_summation_hash_set(
            build_sliding_window(parsed_input.clone(), width, i)
        ).contains(&parsed_input[i+width]);
        if !flag {
            let res = find_sum_subset(parsed_input.clone(), i+width);
            let min_val = res.iter().min().unwrap();
            let max_val = res.iter().max().unwrap();
            return RetType::U128(min_val + max_val);
        }
    }

    RetType::U128(0u128)
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
    fn test_build_summation_hash_set() {
        init();

        let input: Vec<u128> = vec![
            1, 2, 4, 8, 16
        ];

        let res = build_summation_hash_set(input);

        assert!(res.contains(&3u128));
        assert!(res.contains(&5u128));
        assert!(res.contains(&9u128));
        assert!(res.contains(&17u128));
        assert!(res.contains(&6u128));
        assert!(res.contains(&10u128));
        assert!(res.contains(&18u128));
        assert!(res.contains(&12u128));
        assert!(res.contains(&20u128));
        assert!(res.contains(&24u128));
    }

    #[test]
    fn test_build_sliding_window() {
        init();

        let input: Vec<u128> = vec![
            1, 2, 4, 8, 16
        ];

        assert_eq!(build_sliding_window(input, 3, 1), vec![2u128, 4u128, 8u128]);        
    }

    #[test]
    fn test_find_sum_subset() {
        init();

        let input: Vec<u128> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 
            117, 150, 182, 127, 219, 299, 277, 309
        ];

        let idx = 14; // 127

        let res = find_sum_subset(input, idx);

        assert_eq!(res, vec![15, 25, 47, 40]);
    }
}