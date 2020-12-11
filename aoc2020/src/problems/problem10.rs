use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use std::cmp::max;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

fn use_all_adapters(mut adapter_list: Vec<u32>) -> (u32, u32, u32) {
    // let mut wasted_adapters = 0;
    let mut one_jolt = 0;
    let mut two_jolts = 0;
    let mut three_jolts = 0;

    adapter_list.sort();

    let mut current_jolt = 0;
    for adapter in adapter_list {
        match adapter - current_jolt {
            1 => one_jolt += 1,
            2 => two_jolts += 1,
            3 => three_jolts += 1,
            _ => error!("Next adapter has a {} joltage jump!", adapter - current_jolt),
        }
        current_jolt = adapter;
    }

    three_jolts += 1; // For the device's built-in

    (one_jolt, two_jolts, three_jolts)
}

fn adapter_partition(mut adapter_list: Vec<u32>) -> u128 {

    // Add first and last to the list
    adapter_list.push(0);
    adapter_list.sort();
    adapter_list.push(adapter_list[adapter_list.len()-1]+3);

    let mut num_paths = vec![0; adapter_list.len()];
    num_paths[0] = 1; // Start with main path - all the adapters!

    for i in 1..adapter_list.len() {
        for j in max(0, i as i32 - 3) as usize..i { // Look back at last 3
            if adapter_list[i] - adapter_list[j] <= 3 {
                num_paths[i] += num_paths[j];
            }
        }
    }

    num_paths[adapter_list.len()-1]
}

/// Problem #10, part 1
/// The first step of attacking the weakness in the XMAS data is to find the 
///  first number in the list (after the preamble) which is not the sum of two 
///  of the 25 numbers before it. What is the first number that does not have 
///  this property?
pub fn problem_101(input: Vec<String>) -> u128 {
    let parsed_input: Vec<u32> = input
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
    ;

    let res = use_all_adapters(parsed_input);

    debug!("Adapters: {:?}", res);

    res.0 as u128 * res.2 as u128
}

/// Problem #10, part 2
/// What is the total number of distinct ways you can arrange the adapters to 
///  connect the charging outlet to your device?
pub fn problem_102(input: Vec<String>) -> u128 {
    let parsed_input: Vec<u32> = input
        .into_iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
    ;

    adapter_partition(parsed_input)
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
    /// In this example, when using every adapter, there are 7 differences of 
    ///  1 jolt and 5 differences of 3 jolts.
    fn test_use_all_adapters() {
        init();

        let input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        let res = use_all_adapters(input);

        assert_eq!(res.0, 7);
        assert_eq!(res.2, 5);
    }

    #[test]
    /// In this larger example, in a chain that uses all of the adapters, 
    ///  there are 22 differences of 1 jolt and 10 differences of 3 jolts.
    fn test_use_all_adapters_2() {
        init();

        let input = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 
          45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];

        let res = use_all_adapters(input);

        assert_eq!(res.0, 22);
        assert_eq!(res.2, 10);
    }

    #[test]
    fn test_adapter_partition() {
        init();

        // 0 [1, 2, 3] 6
        // 0 [1, 3]    6
        // 0 [2, 3]    6
        // 0 [3]       6
        let input = vec![1,2,3];

        let res = adapter_partition(input);

        assert_eq!(res, 4);
    }

    #[test]
    fn test_adapter_partition_2() {
        init();

        // Sorted: 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19
        let input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        let res = adapter_partition(input);

        assert_eq!(res, 8);
    }

    #[test]
    fn test_adapter_partition_3() {
        init();

        let input = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 
          45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];

        let res = adapter_partition(input);

        assert_eq!(res, 19208);
    }
}