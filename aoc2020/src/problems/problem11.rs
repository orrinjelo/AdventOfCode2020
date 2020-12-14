use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

fn count_occupied_neighbors(state: &Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    let mut s = 0;
    let max_i = state.len()-1;
    let max_j = state[0].len()-1;
 
    s += if i!=0     { if j!=0     { if state[i-1][j-1] == 2 {1} else {0} } else {0} } else {0} +
         if i!=0     {               if state[i-1][j]   == 2 {1} else {0}            } else {0} +
         if i!=0     { if j!=max_j { if state[i-1][j+1] == 2 {1} else {0} } else {0} } else {0} +
 
                       if j!=0     { if state[i][j-1]   == 2 {1} else {0} } else {0}            +
                       if j!=max_j { if state[i][j+1]   == 2 {1} else {0} } else {0}            +
 
         if i!=max_i { if j!=0     { if state[i+1][j-1] == 2 {1} else {0} } else {0} } else {0} +
         if i!=max_i {               if state[i+1][j]   == 2 {1} else {0}            } else {0} +
         if i!=max_i { if j!=max_j { if state[i+1][j+1] == 2 {1} else {0} } else {0} } else {0}
    ;

    s
}

fn check_direction(state: &Vec<Vec<u32>>, i: usize, j: usize, ii: i32, jj: i32, mul: i32) -> bool {
    let i_len = state.len();
    let j_len = state[0].len();

    trace!("({},{}) In direction {}, {}...", i, j, ii*mul, jj*mul);
    trace!("state size: {} x {}", state.len(), state[0].len());

    if i as i32 + ii*mul < 0 { // 0 boundary condition
        trace!("Hit i zero-boundary.");
        return false;
    } else if ii*mul + i as i32 >= i_len as i32 { // Upper boundary condition
        trace!("Hit i pos-boundary.");
        return false;
    } else if j as i32 + jj*mul < 0 { // 0 boundary condition
        trace!("Hit j zero-boundary.");
        return false;
    } else if jj*mul + j as i32 >= j_len as i32 { // Upper boundary condition
        trace!("Hit j pos-boundary.");
        return false;
    }
    if state[(i as i32+ii*mul) as usize][(j as i32+jj*mul) as usize] == 2 {
        return true;
    } else if state[(i as i32+ii*mul) as usize][(j as i32+jj*mul) as usize] == 0 {
        return check_direction(state, i, j, ii, jj, mul+1);
    } else {
        trace!("Hit empty chair: {}", state[(i as i32+ii*mul) as usize][(j as i32+jj*mul) as usize]);
        return false
    }
}

fn count_occupied_distant_neighbors(state: &Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    let mut s = 0;
 
    s += if check_direction(state, i, j, -1, -1, 1) {1} else {0}
      +  if check_direction(state, i, j, -1,  0, 1) {1} else {0}
      +  if check_direction(state, i, j, -1,  1, 1) {1} else {0}
      +  if check_direction(state, i, j,  0,  1, 1) {1} else {0}
      +  if check_direction(state, i, j,  1,  1, 1) {1} else {0}
      +  if check_direction(state, i, j,  1,  0, 1) {1} else {0}
      +  if check_direction(state, i, j,  1, -1, 1) {1} else {0}
      +  if check_direction(state, i, j,  0, -1, 1) {1} else {0};

    s
}
#[allow(dead_code)]
fn print_chairs(state: &Vec<Vec<u32>>) {
    let i_len = state.len();
    let j_len = state[0].len();

    for i in 0..i_len {
        for j in 0..j_len {
            print!("{}", state[i][j]);
        }
        println!();
    }
    println!();
}

fn count_occupied_chairs(state: &Vec<Vec<u32>>) -> u32 {
    let i_len = state.len();
    let j_len = state[0].len();

    let mut s = 0;
    for i in 0..i_len {
        for j in 0..j_len {
            if state[i][j] == 2 {
                s += 1;
            }
        }
    }
    s
}

/// If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
///
/// If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
///
/// Otherwise, the seat's state does not change.
/// 
/// Floor (.) never changes; seats don't move, and nobody sits on the floor.
fn game_of_chairs(state: &mut Vec<Vec<u32>>) -> bool {
    let i_len = state.len();
    let j_len = state[0].len();

    let mut old_state = vec![vec![0; j_len]; i_len];

    for i in 0..i_len {
        for j in 0..j_len {
            old_state[i][j] = state[i][j];
        }
    }
    
    for i in 0..i_len {
        for j in 0..j_len {
            state[i][j] = match old_state[i][j] {
                0 => continue,
                1 => {
                    if count_occupied_neighbors(&old_state, i, j) == 0 {
                        2
                    } else {1}
                },
                2 => {
                    if count_occupied_neighbors(&old_state, i, j) >= 4 {
                        1
                    } else {2}
                },
                x => x,
            }
        }
    }

    old_state == *state
}

/// If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
///
/// If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
///
/// Otherwise, the seat's state does not change.
/// 
/// Floor (.) never changes; seats don't move, and nobody sits on the floor.
fn game_of_swedish_chairs(state: &mut Vec<Vec<u32>>) -> bool {
    let i_len = state.len();
    let j_len = state[0].len();

    let mut old_state = vec![vec![0; j_len]; i_len];

    for i in 0..i_len {
        for j in 0..j_len {
            old_state[i][j] = state[i][j];
        }
    }
    
    for i in 0..i_len {
        for j in 0..j_len {
            state[i][j] = match old_state[i][j] {
                0 => 0,
                1 => {
                    if count_occupied_distant_neighbors(&old_state, i, j) == 0 {2} else {1}
                },
                2 => {
                    if count_occupied_distant_neighbors(&old_state, i, j) >= 5 {1} else {2}
                },
                x => x
            };
        }
    }

    old_state == *state
}

/// Problem #11, part 1
pub fn problem_111(input: Vec<String>) -> u32 {
    let mut parsed_input: Vec<Vec<u32>> = vec![vec![0; input[9].len()]; input.len()];

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            parsed_input[i][j] = match input[i].chars().nth(j) {
                Some('.') => 0,
                Some('L') => 1,
                Some('#') => 2,
                Some(x) => {
                    warn!("Invalid character found: {}", x);
                    0
                },
                None => {
                    error!("None condition in extracting character...");
                    0
                }
            }
        }
    }

    while !game_of_chairs(&mut parsed_input) {
        // print_chairs(&parsed_input);
    }

    count_occupied_chairs(&parsed_input)
}

/// Problem #11, part 2
pub fn problem_112(input: Vec<String>) -> u32 {
    let mut parsed_input: Vec<Vec<u32>> = vec![vec![0; input[9].len()]; input.len()];

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            parsed_input[i][j] = match input[i].chars().nth(j) {
                Some('.') => 0,
                Some('L') => 1,
                Some('#') => 2,
                Some(x) => {
                    warn!("Invalid character found: {}", x);
                    0
                },
                None => {
                    error!("None condition in extracting character...");
                    0
                }
            }
        }
    }

    while !game_of_swedish_chairs(&mut parsed_input) {
    }

    count_occupied_chairs(&parsed_input)
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
    fn test_step() {
        init();

        let mut input = vec![
            vec![1,0,1,1,0,1,1,0,1,1],
            vec![1,1,1,1,1,1,1,0,1,1],
            vec![1,0,1,0,1,0,0,1,0,0],
            vec![1,1,1,1,0,1,1,0,1,1],
            vec![1,0,1,1,0,1,1,0,1,1],
            vec![1,0,1,1,1,1,1,0,1,1],
            vec![0,0,1,0,1,0,0,0,0,0],
            vec![1,1,1,1,1,1,1,1,1,1],
            vec![1,0,1,1,1,1,1,1,0,1],
            vec![1,0,1,1,1,1,1,0,1,1],
        ];

        let res = game_of_chairs(& mut input);

        assert!(!res);

        let output = vec![
            vec![2,0,2,2,0,2,2,0,2,2],
            vec![2,2,2,2,2,2,2,0,2,2],
            vec![2,0,2,0,2,0,0,2,0,0],
            vec![2,2,2,2,0,2,2,0,2,2],
            vec![2,0,2,2,0,2,2,0,2,2],
            vec![2,0,2,2,2,2,2,0,2,2],
            vec![0,0,2,0,2,0,0,0,0,0],
            vec![2,2,2,2,2,2,2,2,2,2],
            vec![2,0,2,2,2,2,2,2,0,2],
            vec![2,0,2,2,2,2,2,0,2,2],
        ];

        assert_eq!(input, output);
    }

    #[test]
    fn test_check_direction() {
        init();

        // 3, 4
        let input = vec![
            vec![2,2,2,2,0,2,2,0,2,2],
            vec![2,2,0,2,0,2,2,0,2,2],
            vec![2,0,2,0,0,0,0,2,0,0],
            vec![2,2,2,2,0,2,2,0,2,2],
            vec![2,0,2,0,0,0,2,0,2,2],
            vec![2,0,0,2,2,2,0,0,2,2],
            vec![0,0,2,0,2,0,0,0,0,0],
            vec![2,2,2,2,2,2,2,2,0,2],
            vec![2,0,2,2,2,2,2,2,0,0],
            vec![2,0,2,2,2,2,2,0,2,2],
        ];

        assert!( check_direction(&input, 3, 4, -1, -1, 1));
        assert!(!check_direction(&input, 3, 4, -1,  0, 1));
        assert!( check_direction(&input, 3, 4, -1,  1, 1));
        assert!( check_direction(&input, 3, 4,  0,  1, 1));
        assert!(!check_direction(&input, 3, 4,  1,  1, 1));
        assert!( check_direction(&input, 3, 4,  1,  0, 1));
        assert!( check_direction(&input, 3, 4,  1, -1, 1));
        assert!( check_direction(&input, 3, 4,  0, -1, 1));

        assert_eq!( count_occupied_distant_neighbors(&input, 3, 4), 6);
    }

    #[test]
    fn test_swiss_step() {
        init();

        let mut input = vec![
            vec![2,0,1,2,0,1,2,0,1,2],
            vec![2,1,1,1,1,1,1,0,1,1],
            vec![1,0,1,0,1,0,0,2,0,0],
            vec![2,2,1,2,0,2,1,0,1,2],
            vec![1,0,1,2,0,2,1,0,1,2],
            vec![2,0,1,2,2,2,2,0,1,1],
            vec![0,0,2,0,2,0,0,0,0,0],
            vec![1,1,1,2,2,2,1,1,1,2],
            vec![2,0,1,1,1,1,1,2,0,1],
            vec![2,0,1,2,1,1,2,0,1,2],
        ];

        let res = game_of_swedish_chairs(& mut input);

        assert!(!res);

        let output = vec![
            vec![2,0,1,2,0,1,2,0,1,2],
            vec![2,1,1,1,1,1,1,0,1,1],
            vec![1,0,1,0,1,0,0,2,0,0],
            vec![2,2,1,2,0,2,1,0,1,2],
            vec![1,0,1,2,0,1,1,0,1,2],
            vec![2,0,1,1,1,1,2,0,1,1],
            vec![0,0,2,0,1,0,0,0,0,0],
            vec![1,1,1,2,2,2,1,1,1,2],
            vec![2,0,1,1,1,1,1,2,0,1],
            vec![2,0,1,2,1,1,2,0,1,2],
        ];

        print_chairs(&input);
        print_chairs(&output);

        assert_eq!(input, output);
    }

    #[test]
    fn test_swiss_step_2() {
        init();

        let mut state = vec![
            vec![1,0,1,1,0,1,1,0,1,1],
            vec![1,1,1,1,1,1,1,0,1,1],
            vec![1,0,1,0,1,0,0,1,0,0],
            vec![1,1,1,1,0,1,1,0,1,1],
            vec![1,0,1,1,0,1,1,0,1,1],
            vec![1,0,1,1,1,1,1,0,1,1],
            vec![0,0,1,0,1,0,0,0,0,0],
            vec![1,1,1,1,1,1,1,1,1,1],
            vec![1,0,1,1,1,1,1,1,0,1],
            vec![1,0,1,1,1,1,1,0,1,1],
        ];

        game_of_swedish_chairs(& mut state);

        let step1 = vec![
            vec![2,0,2,2,0,2,2,0,2,2],
            vec![2,2,2,2,2,2,2,0,2,2],
            vec![2,0,2,0,2,0,0,2,0,0],
            vec![2,2,2,2,0,2,2,0,2,2],
            vec![2,0,2,2,0,2,2,0,2,2],
            vec![2,0,2,2,2,2,2,0,2,2],
            vec![0,0,2,0,2,0,0,0,0,0],
            vec![2,2,2,2,2,2,2,2,2,2],
            vec![2,0,2,2,2,2,2,2,0,2],
            vec![2,0,2,2,2,2,2,0,2,2],
        ];

        assert_eq!(state, step1);

        game_of_swedish_chairs(& mut state);

        let step2 = vec![
            vec![2,0,1,1,0,1,1,0,1,2],
            vec![2,1,1,1,1,1,1,0,1,1],
            vec![1,0,1,0,1,0,0,1,0,0],
            vec![1,1,1,1,0,1,1,0,1,1],
            vec![1,0,1,1,0,1,1,0,1,1],
            vec![1,0,1,1,1,1,1,0,1,1],
            vec![0,0,1,0,1,0,0,0,0,0],
            vec![1,1,1,1,1,1,1,1,1,2],
            vec![2,0,1,1,1,1,1,1,0,1],
            vec![2,0,1,1,1,1,1,0,1,2],
        ];

        assert_eq!(state, step2);

        game_of_swedish_chairs(& mut state);

        let step3 = vec![
            vec![2,0,1,2,0,2,2,0,1,2],
            vec![2,1,2,2,2,2,2,0,1,1],
            vec![1,0,2,0,2,0,0,2,0,0],
            vec![2,2,1,2,0,2,2,0,2,2],
            vec![2,0,2,2,0,2,1,0,2,2],
            vec![2,0,2,2,2,2,2,0,2,1],
            vec![0,0,2,0,2,0,0,0,0,0],
            vec![1,1,1,2,2,2,2,1,1,2],
            vec![2,0,1,2,2,2,2,2,0,1],
            vec![2,0,1,2,2,2,2,0,1,2],
        ];

        assert_eq!(state, step3);

    }

    #[test]
    fn test_distant_neighbors() {
        init();

        let input = vec![
            vec![0,2,2,0,2,2,0],
            vec![2,0,2,0,2,0,2],
            vec![2,2,0,0,0,2,2],
            vec![0,0,0,1,0,0,0],
            vec![2,2,0,0,0,2,2],
            vec![2,0,2,0,2,0,2],
            vec![0,2,2,0,2,2,0],
        ];

        assert_eq!(count_occupied_distant_neighbors(&input, 3, 3), 0);

        let input1 = vec![
            vec![0,2,2,0,2,2,2],
            vec![2,0,2,0,2,0,2],
            vec![2,2,0,0,0,2,2],
            vec![0,0,0,1,0,0,0],
            vec![2,2,0,0,0,2,2],
            vec![2,0,2,0,2,0,2],
            vec![0,2,2,0,2,2,0],
        ];

        assert_eq!(count_occupied_distant_neighbors(&input1, 3, 3), 1);

        let input2 = vec![
            vec![0,2,2,0,2,2,2],
            vec![2,0,2,0,2,0,2],
            vec![2,2,0,0,0,2,2],
            vec![0,0,0,1,0,0,2],
            vec![2,2,0,0,0,2,2],
            vec![2,0,2,0,2,0,2],
            vec![0,2,2,0,2,2,0],
        ];

        assert_eq!(count_occupied_distant_neighbors(&input2, 3, 3), 2);

        let input3 = vec![
            vec![0,2,2,0,2,2,2],
            vec![2,0,2,0,2,0,2],
            vec![2,2,0,0,0,2,2],
            vec![0,0,0,1,0,0,2],
            vec![2,2,0,0,0,2,2],
            vec![2,0,2,0,2,0,2],
            vec![0,2,2,0,2,2,2],
        ];

        assert_eq!(count_occupied_distant_neighbors(&input3, 3, 3), 3);

        let input4 = vec![
            vec![0,2,2,0,2,2,2],
            vec![2,0,2,0,2,0,2],
            vec![2,2,0,0,0,2,2],
            vec![0,0,0,1,0,0,2],
            vec![2,2,0,0,0,2,2],
            vec![2,0,2,0,2,0,2],
            vec![0,2,2,2,2,2,2],
        ];

        assert_eq!(count_occupied_distant_neighbors(&input4, 3, 3), 4);

        let input5 = vec![
            vec![0,2,2,0,2,2,2],
            vec![2,0,2,0,2,0,2],
            vec![2,2,0,0,0,2,2],
            vec![0,0,0,1,0,0,2],
            vec![2,2,0,0,0,2,2],
            vec![2,0,2,0,2,0,2],
            vec![2,2,2,2,2,2,2],
        ];

        assert_eq!(count_occupied_distant_neighbors(&input5, 3, 3), 5);

        let input6 = vec![
            vec![0,2,2,0,2,2,2],
            vec![2,0,2,0,2,0,2],
            vec![2,2,0,0,0,2,2],
            vec![2,0,0,1,0,0,2],
            vec![2,2,0,0,0,2,2],
            vec![2,0,2,0,2,0,2],
            vec![2,2,2,2,2,2,2],
        ];

        assert_eq!(count_occupied_distant_neighbors(&input6, 3, 3), 6);

        let input7 = vec![
            vec![2,2,2,0,2,2,2],
            vec![2,0,2,0,2,0,2],
            vec![2,2,0,0,0,2,2],
            vec![2,0,0,1,0,0,2],
            vec![2,2,0,0,0,2,2],
            vec![2,0,2,0,2,0,2],
            vec![2,2,2,2,2,2,2],
        ];

        assert_eq!(count_occupied_distant_neighbors(&input7, 3, 3), 7);

        let input8 = vec![
            vec![2,2,2,2,2,2,2],
            vec![2,0,2,0,2,0,2],
            vec![2,2,0,0,0,2,2],
            vec![2,0,0,1,0,0,2],
            vec![2,2,0,0,0,2,2],
            vec![2,0,2,0,2,0,2],
            vec![2,2,2,2,2,2,2],
        ];

        assert_eq!(count_occupied_distant_neighbors(&input8, 3, 3), 8);

        let input9 = vec![
            vec![1,2,2,1,2,2,1],
            vec![2,0,2,0,2,0,2],
            vec![2,2,0,0,0,2,2],
            vec![1,0,0,1,0,0,1],
            vec![2,2,0,0,0,2,2],
            vec![2,0,2,0,2,0,2],
            vec![1,2,2,1,2,2,1],
        ];

        assert_eq!(count_occupied_distant_neighbors(&input9, 0, 0), 2);
        assert_eq!(count_occupied_distant_neighbors(&input9, 6, 0), 2);
        assert_eq!(count_occupied_distant_neighbors(&input9, 0, 6), 2);
        assert_eq!(count_occupied_distant_neighbors(&input9, 6, 6), 2);
        assert_eq!(count_occupied_distant_neighbors(&input9, 3, 0), 4);
        assert_eq!(count_occupied_distant_neighbors(&input9, 3, 6), 4);
        assert_eq!(count_occupied_distant_neighbors(&input9, 6, 3), 4);
        assert_eq!(count_occupied_distant_neighbors(&input9, 0, 3), 4);

        let input10 = vec![
            vec![0,0,0,0,0,0,0,2,0],
            vec![0,0,0,2,0,0,0,0,0],
            vec![0,2,0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0],
            vec![0,0,2,1,0,0,0,0,2],
            vec![0,0,0,0,2,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0],
            vec![2,0,0,0,0,0,0,0,0],
            vec![0,0,0,2,0,0,0,0,0],
        ];

        assert_eq!(count_occupied_distant_neighbors(&input10, 4, 3), 8);

        let input11 = vec![
            vec![0,0,0,0,0,0,0,0,0,0,0,0,0],
            vec![0,1,0,1,0,2,0,2,0,2,0,2,0],
            vec![0,0,0,0,0,0,0,0,0,0,0,0,0],
        ];

        assert_eq!(count_occupied_distant_neighbors(&input11, 1, 1), 0);
        assert_eq!(count_occupied_distant_neighbors(&input11, 1, 3), 1);
    }

}