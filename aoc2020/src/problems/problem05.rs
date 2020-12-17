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

#[allow(dead_code)]
/// Boarding pass
/// Has a row, column, binary space partitioning code (bsp), and ID.
/// Row and column can be computed from BSP, and ID can be calculated from that
struct BoardingPass {
    bsp: String,
    row: u32,
    col: u32,
    pub id: u32
}

/// Binary split, if F/L, first half; if B/R, back half.
fn binary_split(range: (u32, u32), bsp_code: String) -> u32 {
    // So...ah...we could just convert the string into binary instead of this crap.
    binary_split_indexed(range, bsp_code, 0).0
}

fn binary_split_indexed(range: (u32, u32), bsp_code: String, bsp_index: usize) -> (u32, u32) {
    let mut new_range = (range.0, range.1);
    trace!("range: {:?}, bsp: {}", range, bsp_index);
    let code = bsp_code.chars().nth(bsp_index).unwrap();
    match code {
        'F' | 'L' => new_range = (new_range.0, new_range.0+(new_range.1-new_range.0)/2),
        'B' | 'R' => new_range = (new_range.0+(new_range.1-new_range.0)/2+1, new_range.1),
        _ => return new_range
    }
    if new_range.0 != new_range.1 {
        new_range = binary_split_indexed(new_range, bsp_code, bsp_index+1);
    } 
    return new_range;
}

impl BoardingPass {
    fn new(bsp_str: String) -> BoardingPass {
        let row_str = &bsp_str[0..7];
        let col_str = &bsp_str[7..10];

        trace!("rowstr: {}, colstr: {}", row_str, col_str);

        let row_val = binary_split((0,127), row_str.to_string());
        let col_val = binary_split((0,7), col_str.to_string());

        BoardingPass {
            bsp: bsp_str,
            row: row_val,
            col: col_val,
            id: col_val + row_val * 8,
        }
    }

}

/// Problem #05, Part 1
/// What is the highest seat ID on a boarding pass?
pub fn problem_051(input: Vec<String>) -> RetType {
    let mut max_id = 0;
    for entry in input {
         let pass = BoardingPass::new(entry);
         if pass.id > max_id {
            max_id = pass.id;
         }
    }

    return RetType::U32(max_id);
}

/// Problem #05, Part 2
/// It's a completely full flight, so your seat should be the only missing boarding pass in your 
///  list. However, there's a catch: some of the seats at the very front and back of the plane 
///  don't exist on this aircraft, so they'll be missing from your list as well.
///
/// Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours 
///  will be in your list.
///
/// What is the ID of your seat?
pub fn problem_052(input: Vec<String>) -> RetType {
    let mut pass_vector = Vec::new();
    for entry in input {
        let pass = BoardingPass::new(entry);
        pass_vector.push(pass.id);
    }

    pass_vector.sort();

    trace!("Pass Vector: {:?}", pass_vector);

    let mut last_id = 0;
    for pass_id in pass_vector {
        if last_id == 0 {
            last_id = pass_id;
        } else if last_id + 1 != pass_id {
            return RetType::U32(last_id + 1);
        } else {
            last_id = pass_id;
        }
    }
    return RetType::U32(0);
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
    /// BFFFBBFRRR: row 70, column 7, seat ID 567.
    /// FFFBBBFRRR: row 14, column 7, seat ID 119.
    /// BBFFBBFRLL: row 102, column 4, seat ID 820.
    fn test_binary_split() {
        init();

        // FBFBBFFRLR row 44, column 5
        assert_eq!(binary_split((0,127), "FBFBBFF".to_string()), 44);
        assert_eq!(binary_split((0,7),   "RLR".to_string()),      5);

        assert_eq!(binary_split((0,127), "BFFFBBF".to_string()), 70);
        assert_eq!(binary_split((0,7),   "RRR".to_string()),      7);

        assert_eq!(binary_split((0,127), "FFFBBBF".to_string()), 14);
        assert_eq!(binary_split((0,7),   "RRR".to_string()),      7);

        assert_eq!(binary_split((0,127), "BBFFBBF".to_string()), 102);
        assert_eq!(binary_split((0,7),   "RLL".to_string()),      4);
    }

    #[test]
    fn test_boarding_pass() {
        init();
        
        let boarding_pass_1 = BoardingPass::new("BFFFBBFRRR".to_string());

        assert_eq!(boarding_pass_1.bsp, "BFFFBBFRRR".to_string());
        assert_eq!(boarding_pass_1.row, 70);
        assert_eq!(boarding_pass_1.col, 7);
        assert_eq!(boarding_pass_1.id, 567);
    }
}