use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

/// ArboralLandscape
/// Due to the local geology, trees in this area only grow on exact integer coordinates 
///  in a grid. You make a map (your puzzle input) of the open squares (.) and trees (#) 
///  you can see.
/// These aren't the only trees, though; due to something you read about once involving 
///  arboreal genetics and biome stability, the same pattern repeats to the right many times.
pub struct ArboralLandscape {
   tree_map: Vec<Vec<u8>>, 
}

impl ArboralLandscape {
    pub fn new(string_map: Vec<String>) -> ArboralLandscape {
        let rows = string_map.len();
        let cols = string_map[0].len();

        trace!("rows: {}", rows);
        trace!("cols: {}", cols);

        let mut tm = vec![vec![0; cols]; rows];

        trace!("tm rows: {}", tm.len());
        trace!("tm cols: {}", tm[0].len());

        for row_num in 0..rows {
            for col_num in 0..cols {
                if string_map[row_num].chars().nth(col_num).unwrap() == '#' {
                    tm[row_num][col_num] = 1;
                }
            }
        }
        ArboralLandscape {
            tree_map: tm,
        }
    }

    #[allow(dead_code)]
    pub fn print_n(&self, repeat_n: usize) {
        for row in &self.tree_map {
            for c in 0..row.len()*repeat_n {
                match row[c%row.len()] {
                    0u8 => {
                        print!(".");
                    },
                    1u8 => {
                        print!("#");
                    },
                    2u8 => {
                        print!("O");
                    },
                    3u8 => {
                        print!("X");
                    },
                    _ => {
                        print!("?");
                    }
                }
            }
            println!();
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        self.print_n(1);
    }

    pub fn traverse(&mut self, down: u8, right: u8) -> u128 {
        let mut tree_count: u128 = 0;
        let mut position = (0, 0);
        let row_len = self.tree_map[0].len();
        while position.0 < self.tree_map.len() {
            tree_count += self.tree_map[position.0][position.1 % row_len] as u128;
            position = (position.0 + down as usize, position.1 + right as usize);
        }
        return tree_count;
    }
}

/**
 *  Problem #03, part 1
 *  With the toboggan login problems resolved, you set off toward the 
 *  airport. While travel by toboggan might be easy, it's certainly not safe: 
 *  there's very minimal steering and the area is covered in trees. You'll 
 *  need to see which angles will take you near the fewest trees.
 * Due to the local geology, trees in this area only grow on exact integer 
 *  coordinates in a grid.
 */
pub fn problem_031(input: Vec<String>) -> u128 {
    let mut arboral_landscape = ArboralLandscape::new(input);
    return arboral_landscape.traverse(1,3);
}

/**
 *  Problem #03, part 2
 *  Time to check the rest of the slopes - you need to minimize
 *  the probability of a sudden arboreal stop, after all.
 * Determine the number of trees you would encounter if, for each of the 
 *  following slopes, you start at the top-left corner and traverse the map 
 *  all the way to the bottom.
 * What do you get if you multiply together the number of trees encountered 
 *  on each of the listed slopes?
 */
pub fn problem_032(input: Vec<String>) -> u128 {
    let mut arboral_landscape = ArboralLandscape::new(input);
    let slope_1_1 = arboral_landscape.traverse(1,1);
    debug!("slope_1_1: {}", slope_1_1);
    let slope_1_3 = arboral_landscape.traverse(1,3);
    debug!("slope_1_3: {}", slope_1_3);
    let slope_1_5 = arboral_landscape.traverse(1,5);
    debug!("slope_1_5: {}", slope_1_5);
    let slope_1_7 = arboral_landscape.traverse(1,7);
    debug!("slope_1_7: {}", slope_1_7);
    let slope_2_1 = arboral_landscape.traverse(2,1);
    debug!("slope_2_1: {}", slope_2_1);
    // arboral_landscape.print_n(5);
    return slope_1_1 * slope_1_3 * slope_1_5 * slope_1_7 * slope_2_1;
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
    fn test_new_and_print_arboral_landscape() {
        init();
        let string_map = vec![
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string()
        ];
        let arboral_landscape = ArboralLandscape::new(string_map);
        arboral_landscape.print_n(6);
    }

    #[test]
    /// Right 1, down 1.
    /// Right 3, down 1.
    /// Right 5, down 1.
    /// Right 7, down 1.
    /// Right 1, down 2.
    /// 
    /// In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s) respectively; 
    /// multiplied together, these produce the answer 336.
    fn test_traverse_arboral_landscape() {
        init();
        let string_map = vec![
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string()
        ];
        let mut arboral_landscape = ArboralLandscape::new(string_map);
        assert_eq!(arboral_landscape.traverse(1,1), 2);
        assert_eq!(arboral_landscape.traverse(1,3), 7);
        assert_eq!(arboral_landscape.traverse(1,5), 3);
        assert_eq!(arboral_landscape.traverse(1,7), 4);
        assert_eq!(arboral_landscape.traverse(2,1), 2);
    }

    #[test]
    fn test_traverse_arboral_landscape_part_2() {
        init();
        let string_map = vec![
            "....".to_string(),
            "....".to_string(),
            ".#..".to_string()
        ];
        let mut arboral_landscape = ArboralLandscape::new(string_map);
        assert_eq!(arboral_landscape.traverse(2,1), 1);
    }
}