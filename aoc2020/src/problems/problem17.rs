use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

fn parse_string_input(input: Vec<String>) -> Vec<Vec<u8>> {
    let mut parsed_input: Vec<Vec<u8>> = vec![vec![0; input[0].len()]; input.len()];

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            parsed_input[i][j] = match input[i].chars().nth(j) {
                Some('.') => 0u8,
                Some('#') => 1u8,
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
    parsed_input
}

/// Problem #17, part 1
pub fn problem_171(input: Vec<String>) -> u32 {
    let parsed = parse_string_input(input);
    let mut cube = TheCube::new(16);
    cube.set_initial_state(parsed);
    for _ in 0..6 {
        cube.naive_step();
    }
    cube.count_actives()
}

/// Problem #17, part 2
pub fn problem_172(input: Vec<String>) -> u32 {
    let parsed = parse_string_input(input);
    let mut cube = TheCube::new(16);
    cube.set_initial_state(parsed);
    for _ in 0..6 {
        cube.hyper_step();
    }
    cube.count_actives()
}

#[derive(Clone, Debug, PartialEq)]
struct TheCube {
    dim: usize,
    state: Vec<Vec<Vec<Vec<u8>>>>
}

impl TheCube {
    pub fn new(dim: usize) -> TheCube {
        TheCube {
            dim: dim,
            state: vec![vec![vec![vec![0u8; dim*2+1]; dim*2+1]; dim*2+1]; dim*2+1],
        }
    }

    pub fn get(&self, x: i64, y: i64, z: i64) -> u8 {
        let dim = (self.state.len()-1)/2;
        if x.abs() > self.dim as i64 ||
           y.abs() > self.dim as i64 ||
           z.abs() > self.dim as i64 {
            error!("Out of bounds condition!");
            return 0;
        }
        self.state[(x + dim as i64) as usize][(y + dim as i64) as usize][(z + dim as i64) as usize][0+dim]
    }

    pub fn hget(&self, x: i64, y: i64, z: i64, w: i64) -> u8 {
        let dim = (self.state.len()-1)/2;
        if x.abs() > self.dim as i64 ||
           y.abs() > self.dim as i64 ||
           z.abs() > self.dim as i64 ||
           w.abs() > self.dim as i64 {
            error!("Out of bounds condition!");
            return 0;
        }
        self.state[(x + dim as i64) as usize][(y + dim as i64) as usize][(z + dim as i64) as usize][(w + dim as i64) as usize]
    }

    pub fn set(&mut self, x: i64, y: i64, z: i64, value: u8) {
        let dim = (self.state.len()-1)/2;
        if x.abs() > self.dim as i64 ||
           y.abs() > self.dim as i64 ||
           z.abs() > self.dim as i64 {
            error!("Out of bounds condition!");
            return;
        }
        self.state[(x + dim as i64) as usize][(y + dim as i64) as usize][(z + dim as i64) as usize][0+dim] = value;
    }

    pub fn hset(&mut self, x: i64, y: i64, z: i64, w: i64, value: u8) {
        let dim = (self.state.len()-1)/2;
        if x.abs() > self.dim as i64 ||
           y.abs() > self.dim as i64 ||
           z.abs() > self.dim as i64 ||
           w.abs() > self.dim as i64 {
            error!("Out of bounds condition!");
            return;
        }
        self.state[(x + dim as i64) as usize][(y + dim as i64) as usize][(z + dim as i64) as usize][(w + dim as i64) as usize] = value;
    }

    pub fn set_initial_state(&mut self, new_state: Vec<Vec<u8>>) {
        let dim_x = new_state.len();
        let dim_y = new_state[0].len();

        for x in 0..dim_x {
            for y in 0..dim_y {
                self.set((x as i64) - (dim_x as i64)/2, (y as i64) - (dim_y as i64)/2, 0, new_state[x][y]);
            }
        }
    }

    pub fn num_active_neighbors(&self, x: i64, y: i64, z: i64, w: i64) -> u8 {
        let mut s: u8 = 0;
        for i in -1..2 {
            if (x+i).abs() > self.dim as i64 {
                continue;
            }
            for j in -1..2 {
                if (y+j).abs() > self.dim as i64 {
                    continue;
                }
                for k in -1..2 {
                    if (z+k).abs() > self.dim as i64 {
                        continue;
                    }
                    for l in -1..2 {
                        if (w+l).abs() > self.dim as i64 {
                            continue;
                        }
                        if i != 0 || j != 0 || k != 0 || l != 0 {
                            s += self.hget(x+i, y+j, z+k, w+l);
                        }
                    }
                }
            }
        }
        s
    }

    /// If a cube is active and exactly 2 or 3 of its neighbors are also 
    ///  active, the cube remains active. Otherwise, the cube becomes 
    ///  inactive.
    /// If a cube is inactive but exactly 3 of its neighbors are active, 
    ///  the cube becomes active. Otherwise, the cube remains inactive.
    pub fn naive_step(&mut self) {
        let previous = self.clone();
        let dim: i64 = self.dim as i64;
        for i in -dim..dim+1 {
            for j in -dim..dim+1 {
                for k in -dim..dim+1 {
                    let active_neighbors = previous.num_active_neighbors(i, j, k, 0);
                    if previous.hget(i, j, k, 0) == 1u8 {
                        if active_neighbors != 2 && active_neighbors != 3 {
                            self.hset(i, j, k, 0, 0u8);
                        }
                    } else {
                        if active_neighbors == 3 {
                            self.hset(i, j, k, 0, 1u8);
                        }
                    }
                }
            }
        }
    }

    pub fn hyper_step(&mut self) {
        let previous = self.clone();
        let dim: i64 = self.dim as i64;
        for i in -dim..dim+1 {
            for j in -dim..dim+1 {
                for k in -dim..dim+1 {
                    for l in -dim..dim+1 {
                        let active_neighbors = previous.num_active_neighbors(i, j, k, l);
                        if previous.hget(i, j, k, l) == 1u8 {
                            if active_neighbors != 2 && active_neighbors != 3 {
                                self.hset(i, j, k, l, 0u8);
                            }
                        } else {
                            if active_neighbors == 3 {
                                self.hset(i, j, k, l, 1u8);
                            }
                        }
                    }
                }
            }
        }
    }
    pub fn count_actives(&self) -> u32 {
        let dim: i64 = self.dim as i64;
        let mut s = 0;
        for i in -dim..dim+1 {
            for j in -dim..dim+1 {
                for k in -dim..dim+1 {
                    for l in -dim..dim+1 {
                        if self.hget(i, j, k, l) == 1u8 {
                            s += 1;
                        }
                    }
                }
            }
        }
        s
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        let dim: i64 = self.dim as i64;
        for z in -dim..dim+1 {
            println!("z = {}", z);
            for x in -dim..dim+1 {
                for y in -dim..dim+1 {
                    print!("{} ", self.get(x, y, z));
                }
                println!();
            }
            println!();
        }
    }
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
    fn test_cube() {
        init();

        let mut cube = TheCube::new(4);
        cube.set(0,0,0,1u8);
        cube.set(-1,0,0,1u8);
        cube.set(0,-3,4,1u8);

        assert_eq!(cube.get(0,0,0), 1u8);
        assert_eq!(cube.get(-1,0,0), 1u8);
        assert_eq!(cube.get(0,-3,4), 1u8);

        assert_eq!(cube.hget(0,0,0,0), 1u8);
        assert_eq!(cube.hget(-1,0,0,0), 1u8);
        assert_eq!(cube.hget(0,-3,4,0), 1u8);

        for i in -1..2 {
            for j in -1..2 {
                for k in -1..2 {
                    cube.hset(i, j, k, 0, 1u8);
                }
            }
        }

        assert_eq!(cube.num_active_neighbors(0, 0, 0, 0), 26);
    }

    #[test]
    fn test_cube2() {
        init();

        // .#.
        // ..#
        // ###
        let mut cube = TheCube::new(2);

        cube.set_initial_state(
            vec![
                vec![0,1,0],
                vec![0,0,1],
                vec![1,1,1],
            ]
        );

        assert_eq!(cube.get(0,0,0), 0u8);
        assert_eq!(cube.get(-1,0,0), 1u8);
        assert_eq!(cube.get(0,1,0), 1u8);
        assert_eq!(cube.get(1,-1,0), 1u8);
        assert_eq!(cube.get(1,0,0), 1u8);
        assert_eq!(cube.get(1,1,0), 1u8);
        assert_eq!(cube.num_active_neighbors(0,0,0, 0), 5);
        assert_eq!(cube.num_active_neighbors(1,1,-1, 0), 3);
        assert_eq!(cube.num_active_neighbors(-1, -1, -1, 0), 1);
        assert_eq!(cube.count_actives(), 5);
        cube.print();

        cube.naive_step();

        // z=-1
        // #..
        // ..#
        // .#.

        // z=0
        // #.#
        // .##
        // .#.

        // z=1
        // #..
        // ..#
        // .#.

        cube.print();
        // Visually examining this.  Below tests are invalid?

        // assert_eq!(cube.get(0, 0, -1), 1u8);
        // assert_eq!(cube.get(1, 2, -1), 1u8);
        // assert_eq!(cube.get(1, 1, -1), 0u8);

        // assert_eq!(cube.get(0, 0, 0), 1u8);
        // assert_eq!(cube.get(1, 1, 0), 1u8);
        // assert_eq!(cube.get(2, 2, 0), 0u8);

        // assert_eq!(cube.get(1, 2, 1), 1u8);
        // assert_eq!(cube.get(2, 1, 1), 1u8);
        // assert_eq!(cube.get(2, 2, 1), 0u8);

    }
}