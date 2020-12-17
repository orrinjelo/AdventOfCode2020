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

fn get_next_time(current_time: u32, buses: &Vec<u32>) -> (u32, u32) {
    let mut time = current_time;

    loop {
        for bus in buses {
            if time % bus == 0 {
                return (time, *bus);
            }
        }
        time += 1;
        if time == 2400 {
            return (0, buses[0]);
        }
    }
}

// This works...but runs forever
#[allow(dead_code)]
fn chinese_remainder_algo(p: &Vec<u128>, m: &Vec<u128>) -> u128 {
    let mut z = vec![0; p.len()];
    let prod = m.iter().product::<u128>();
    for i in 0..p.len() {
        let g: u128 = prod / m[i];
        z[i] = g;
        let mut j = 1;
        while z[i] % m[i] != p[i] {
            j += 1;
            z[i] = g * j;
        }
    }

    let mut almost = z.iter().sum();
    while almost > prod {
        almost -= prod;
    }

    almost
}

// A cleanup of above?
fn chinese_remainder_algo_2_electric_boogaloo(p: &Vec<u128>, m: &Vec<u128>) -> u128 {
    // let prod = m.iter().product::<u128>();
    let mut step = m[0];
    let mut z = 0u128;

    for i in 1..p.len() {
        while (z + p[i]) % m[i] != 0 {
            z += step;
        }
        step *= m[i];
    }

    z
}


/// Problem #13, part 1
pub fn problem_131(input: Vec<String>) -> RetType {
    let time: u32 = input[0].parse::<u32>().unwrap();
    let buses_str: Vec<&str> = input[1].split(',').collect();
    let mut buses: Vec<u32> = Vec::new();

    for bus in buses_str.iter() {
        match bus.parse::<u32>() {
            Ok(v) => buses.push(v),
            Err(_) => {},
        }
    }

    let next_time = get_next_time(time, &buses);

    RetType::U128(((next_time.0-time) * next_time.1) as u128)
}

/// Problem #13, part 2
pub fn problem_132(input: Vec<String>) -> RetType {

    // Build our m and p
    let buses_str: Vec<&str> = input[1].split(',').collect();
    let mut m: Vec<u128> = Vec::new();
    let mut p: Vec<u128> = Vec::new();

    let mut i = 0;
    for bus in buses_str.iter() {
        match bus.parse::<u128>() {
            Ok(v) => {
                m.push(v);
                // debug!("v: {}, i: {}", v, i);
                p.push(i);
            },
            Err(_) => {},
        }
        i += 1;
    }

    RetType::U128(chinese_remainder_algo_2_electric_boogaloo(&p, &m))
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
    fn test_bus_scheduling() {
        init();

        let current_time: u32 = 939;
        let bus_numbers = vec![7,13,59,31,19];

        let next_time = get_next_time(current_time, &bus_numbers);
        assert_eq!(next_time.0, 944);
        assert_eq!(next_time.1, 59);
    }

    #[test]
    fn test_chinese_remainder_algo() {
        init();

        let p = vec![0, 1];
        let m = vec![7, 13];

        // assert_eq!(chinese_remainder_algo(&p, &m), 77);
        assert_eq!(chinese_remainder_algo_2_electric_boogaloo(&p, &m), 77);

        // 17,x,13,19

        let p2 = vec![0, 2, 3]; // expected modulo
        let m2 = vec![17, 13, 19];

        // assert_eq!(chinese_remainder_algo(&p2, &m2), 3417);
        assert_eq!(chinese_remainder_algo_2_electric_boogaloo(&p2, &m2), 3417);
    }
}