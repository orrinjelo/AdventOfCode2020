use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use std::collections::HashMap;

use plotly::common::Mode;
use plotly::{Plot, Scatter};

fn line_and_scatter_plot(v: &Vec<u32>) {
    let x_axis: Vec<u32> = (0..v.len()).map(|x| x as u32).collect();
    let y_axis: Vec<u32> = v.to_vec();
    let trace1 = Scatter::new(x_axis, y_axis)
        .name("sequence")
        .mode(Mode::Lines);

    let mut plot = Plot::new();

    plot.add_trace(trace1);

    plot.show();
}

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

fn memory_game(start_v: Vec<u32>, stop: usize, plot: bool) -> u32 {
    let mut v: Vec<u32> = Vec::new();

    // Copy start_v
    for i in start_v {
        v.push(i);
    }

    while v.len() != stop {
        let mut iter = v.iter();
        let current_pos: usize = iter.rposition(|&r| r == *v.last().unwrap()).unwrap();
        match iter.rposition(|&r| r == *v.last().unwrap()) {
            Some(x) => {
                v.push((current_pos - x) as u32);
            },
            None => {
                v.push(*v.first().unwrap());
            }
        }
    }

    if plot {
        line_and_scatter_plot(&v);
    }

    *v.last().unwrap()
}

fn ram_memory_game(start_v: Vec<u32>, stop: u32) -> u32 {
    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();

    // Copy start_v
    for i in 0..start_v.len() {
        if let Some(x) = map.get_mut(&start_v[i]) {
            trace!("!!!!!!!!!!!!!!!!!!! Pushing {}", i as u32 + 1);
            x.push(i as u32 + 1);
            if x.len() > 2 {
                x.remove(0);
            }
        } else {
            map.insert(start_v[i], vec![i as u32 + 1]);
        }
    }

    let mut count: u32 = start_v.len() as u32 + 1;
    let mut last: u32 = start_v[start_v.len() - 1];
    // let mut next: u32 = start_v[0];
    trace!("Map: {:?}", map);
    while count != stop+1 {
        trace!("[BEGIN] Count: {}, Last: {}, Map: {:?}", count, last, map);
        let y = map.get(&last);
        match y {
            Some(x) => {
                if x.len() == 2 {
                    last = x[1] - x[0];
                } else {
                    last = start_v[0];
                }
            },
            None => {
                last = start_v[0];
            }
        }
        let z = map.get_mut(&last);
        match z {
            Some(x) => {
                x.push(count);
                if x.len() > 2 {
                    x.remove(0);
                }
            },
            None => {
                map.insert(last, vec![count]);
            }
        }
        trace!("[END] Count: {}, Last: {}, Map: {:?}", count, last, map);
        count += 1;
    }
    trace!("");
    last
}

/// Problem #15, part 1
pub fn problem_151(input: Vec<String>) -> u32 {
    let start_v: Vec<u32> = input.last().unwrap().split(',')
                       .map(|x| x.parse::<u32>().unwrap())
                       .collect();

    memory_game(start_v, 2020, false)
}

/// Problem #15, part 2
pub fn problem_152(input: Vec<String>) -> u32 {
    let start_v: Vec<u32> = input.last().unwrap().split(',')
                       .map(|x| x.parse::<u32>().unwrap())
                       .collect();

    ram_memory_game(start_v, 30000000)
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
    fn test_memory_game() {
        init();

        let v1 = vec![0,3,6];
        let v2 = vec![0,3,6];
        let v3 = vec![0,3,6];
        let v4 = vec![0,3,6];

        assert_eq!(memory_game(v1, 8, false), 0);
        assert_eq!(memory_game(v2, 9, false), 4);
        assert_eq!(memory_game(v3, 10, false), 0);
        assert_eq!(memory_game(v4, 2020, false), 436);
    }

    #[test]
    fn test_memory_game2() {
        init();

        let v1 = vec![0,3,6];
        let v2 = vec![0,3,6];
        let v3 = vec![0,3,6];
        let v4 = vec![0,3,6];

        assert_eq!(ram_memory_game(v1, 8), 0);
        assert_eq!(ram_memory_game(v2, 9), 4);
        assert_eq!(ram_memory_game(v3, 10), 0);
        assert_eq!(ram_memory_game(v4, 2020), 436);
    }

    #[test]
    fn test_plotly() {
        let _v = vec![0,3,6];

        // line_and_scatter_plot(v);
    }
}