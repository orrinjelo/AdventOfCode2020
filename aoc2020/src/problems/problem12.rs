use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Waypoint {
    lat: i32,
    lon: i32,
}

struct Ship {
    lat: i32,
    lon: i32,
    dir: Direction,
    waypoint: Waypoint,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            lat: 0,
            lon: 0,
            dir: Direction::East,
            waypoint: Waypoint {
                lat: 1,
                lon: 10,
            }
        }
    }

    // Action N means to move north by the given value.
    // Action S means to move south by the given value.
    // Action E means to move east by the given value.
    // Action W means to move west by the given value.
    // Action L means to turn left the given number of degrees.
    // Action R means to turn right the given number of degrees.
    // Action F means to move forward by the given value in the direction the 
    //   ship is currently facing.
    fn parse_instruction_naive(&mut self, line: String) {
        let letter: char = line.chars().next().unwrap() as char;
        let value: i32 = line.trim_matches(letter).parse::<i32>().unwrap();

        match letter {
            'N' => { self.lat += value; },
            'S' => { self.lat -= value; },
            'E' => { self.lon += value; },
            'W' => { self.lon -= value; },
            'L' => {
                self.dir = match value {
                    90 => match self.dir {
                        Direction::North => Direction::West,
                        Direction::West  => Direction::South,
                        Direction::South => Direction::East,
                        Direction::East  => Direction::North,
                    },
                    180 => match self.dir {
                        Direction::North => Direction::South,
                        Direction::South => Direction::North,
                        Direction::West  => Direction::East,
                        Direction::East  => Direction::West,
                    },
                    270 => match self.dir {
                        Direction::North => Direction::East,
                        Direction::East  => Direction::South,
                        Direction::South => Direction::West,
                        Direction::West  => Direction::North,
                    },
                    _ => self.dir,
                }
            },
            'R' => {
                self.dir = match value {
                    270 => match self.dir {
                        Direction::North => Direction::West,
                        Direction::West  => Direction::South,
                        Direction::South => Direction::East,
                        Direction::East  => Direction::North,
                    },
                    180 => match self.dir {
                        Direction::North => Direction::South,
                        Direction::South => Direction::North,
                        Direction::West  => Direction::East,
                        Direction::East  => Direction::West,
                    },
                    90 => match self.dir {
                        Direction::North => Direction::East,
                        Direction::East  => Direction::South,
                        Direction::South => Direction::West,
                        Direction::West  => Direction::North,
                    },
                    _ => self.dir,
                }
            },
            'F' => { 
                match self.dir {
                    Direction::North => { self.lat += value; },
                    Direction::South => { self.lat -= value; },
                    Direction::East  => { self.lon += value; },
                    Direction::West  => { self.lon -= value; },
                };
            },
            _ => {}
        };
    }

    /// Action N means to move the waypoint north by the given value.
    /// Action S means to move the waypoint south by the given value.
    /// Action E means to move the waypoint east by the given value.
    /// Action W means to move the waypoint west by the given value.
    /// Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
    /// Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
    /// Action F means to move forward to the waypoint a number of times equal to the given value.
    fn parse_instruction(&mut self, line: String) {
        let letter: char = line.chars().next().unwrap() as char;
        let value: i32 = line.trim_matches(letter).parse::<i32>().unwrap();

        let lat = self.waypoint.lat;
        let lon = self.waypoint.lon;

        debug!("Line: {}", line);

        match letter {
            'N' => { self.waypoint.lat += value; },
            'S' => { self.waypoint.lat -= value; },
            'E' => { self.waypoint.lon += value; },
            'W' => { self.waypoint.lon -= value; },
            'L' => match value {
                90 => {
                    self.waypoint.lon = -lat;
                    self.waypoint.lat = lon;
                },
                180 => {
                    self.waypoint.lon = -lon;
                    self.waypoint.lat = -lat;
                },
                270 => {
                    self.waypoint.lon = lat;
                    self.waypoint.lat = -lon;
                },
                _ => {},
            },
            'R' => match value {
                90 => {
                    self.waypoint.lon = lat;
                    self.waypoint.lat = -lon;
                },
                180 => {
                    self.waypoint.lon = -lon;
                    self.waypoint.lat = -lat;
                },
                270 => {
                    self.waypoint.lon = -lat;
                    self.waypoint.lat = lon;
                },
                _ => {},
            },
            'F' => {
                self.lat += self.waypoint.lat * value;
                self.lon += self.waypoint.lon * value;
            },
            _ => {}
        };
    }    

    pub fn manhattan_distance(&self) -> u32 {
        (self.lat.abs() + self.lon.abs()) as u32
    }
}

/// Problem #12, part 1
pub fn problem_121(input: Vec<String>) -> u32 {
    let mut ship = Ship::new();
    for line in input {
        ship.parse_instruction_naive(line);
    }
    ship.manhattan_distance()
}

/// Problem #12, part 2
pub fn problem_122(input: Vec<String>) -> u32 {
    let mut ship = Ship::new();
    for line in input {
        ship.parse_instruction(line);
    }
    ship.manhattan_distance()
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
    fn test_ship() {
        init();

        let mut ship = Ship::new();

        assert_eq!(ship.dir, Direction::East);
        assert_eq!(ship.lat, 0);
        assert_eq!(ship.lon, 0);

        ship.parse_instruction_naive("N12".to_string());

        assert_eq!(ship.dir, Direction::East);
        assert_eq!(ship.lat, 12);
        assert_eq!(ship.lon, 0);

        ship.parse_instruction_naive("E11".to_string());

        assert_eq!(ship.dir, Direction::East);
        assert_eq!(ship.lat, 12);
        assert_eq!(ship.lon, 11);

        ship.parse_instruction_naive("S10".to_string());

        assert_eq!(ship.dir, Direction::East);
        assert_eq!(ship.lat, 2);
        assert_eq!(ship.lon, 11);

        ship.parse_instruction_naive("W10".to_string());

        assert_eq!(ship.dir, Direction::East);
        assert_eq!(ship.lat, 2);
        assert_eq!(ship.lon, 1);

        ship.parse_instruction_naive("F10".to_string());

        assert_eq!(ship.dir, Direction::East);
        assert_eq!(ship.lat, 2);
        assert_eq!(ship.lon, 11);

        ship.parse_instruction_naive("L90".to_string());

        assert_eq!(ship.dir, Direction::North);
        assert_eq!(ship.lat, 2);
        assert_eq!(ship.lon, 11);

        ship.parse_instruction_naive("L90".to_string());

        assert_eq!(ship.dir, Direction::West);
        assert_eq!(ship.lat, 2);
        assert_eq!(ship.lon, 11);

        ship.parse_instruction_naive("R180".to_string());

        assert_eq!(ship.dir, Direction::East);
        assert_eq!(ship.lat, 2);
        assert_eq!(ship.lon, 11);
    }

    #[test]
    fn test_ship2() {
        init();

        let mut ship = Ship::new();

        assert_eq!(ship.dir, Direction::East);
        assert_eq!(ship.lat, 0);
        assert_eq!(ship.lon, 0);
        assert_eq!(ship.waypoint.lat, 1);
        assert_eq!(ship.waypoint.lon, 10);

        ship.parse_instruction("N12".to_string());

        assert_eq!(ship.dir, Direction::East);
        assert_eq!(ship.lat, 0);
        assert_eq!(ship.lon, 0);
        assert_eq!(ship.waypoint.lat, 13);
        assert_eq!(ship.waypoint.lon, 10);

        ship.parse_instruction("E11".to_string());

        assert_eq!(ship.dir, Direction::East);
        assert_eq!(ship.lat, 0);
        assert_eq!(ship.lon, 0);
        assert_eq!(ship.waypoint.lat, 13);
        assert_eq!(ship.waypoint.lon, 21);

        ship.parse_instruction("S10".to_string());

        assert_eq!(ship.dir, Direction::East);
        assert_eq!(ship.lat, 0);
        assert_eq!(ship.lon, 0);
        assert_eq!(ship.waypoint.lat, 3);
        assert_eq!(ship.waypoint.lon, 21);

        ship.parse_instruction("W10".to_string());

        assert_eq!(ship.dir, Direction::East);
        assert_eq!(ship.waypoint.lat, 3);
        assert_eq!(ship.waypoint.lon, 11);

        ship.parse_instruction("F10".to_string());

        assert_eq!(ship.dir, Direction::East);
        assert_eq!(ship.waypoint.lat, 3);
        assert_eq!(ship.waypoint.lon, 11);
        assert_eq!(ship.lat, 30);
        assert_eq!(ship.lon, 110);

        ship.parse_instruction("L90".to_string());

        assert_eq!(ship.dir, Direction::East);
        assert_eq!(ship.waypoint.lat, 11);
        assert_eq!(ship.waypoint.lon, -3);
        assert_eq!(ship.lat, 30);
        assert_eq!(ship.lon, 110);

        ship.parse_instruction("R90".to_string());

        assert_eq!(ship.dir, Direction::East);
        assert_eq!(ship.waypoint.lat, 3);
        assert_eq!(ship.waypoint.lon, 11);

        ship.parse_instruction("R180".to_string());

        assert_eq!(ship.dir, Direction::East);
        assert_eq!(ship.waypoint.lat, -3);
        assert_eq!(ship.waypoint.lon, -11);
    }    

    #[test]

    /// F10 moves the ship to the waypoint 10 times (a total of 100 units east and 10 
    ///  units north), leaving the ship at east 100, north 10. The waypoint stays 10 
    ///  units east and 1 unit north of the ship.
    /// N3 moves the waypoint 3 units north to 10 units east and 4 units north of the
    ///  ship. The ship remains at east 100, north 10.
    /// F7 moves the ship to the waypoint 7 times (a total of 70 units east and 28 
    ///  units north), leaving the ship at east 170, north 38. The waypoint stays 10
    ///  units east and 4 units north of the ship.
    /// R90 rotates the waypoint around the ship clockwise 90 degrees, moving it to 
    ///  4 units east and 10 units south of the ship. The ship remains at east 170, north 38.
    /// F11 moves the ship to the waypoint 11 times (a total of 44 units east and 
    ///  110 units south), leaving the ship at east 214, south 72. The waypoint stays 
    ///  4 units east and 10 units south of the ship.
    ///
    /// After these operations, the ship's Manhattan distance from its starting position 
    ///  is 214 + 72 = 286.
    fn test_part2() {
        init();

        let mut ship = Ship::new();

        ship.parse_instruction("F10".to_string());
        ship.parse_instruction("N3".to_string());
        ship.parse_instruction("F7".to_string());
        ship.parse_instruction("R90".to_string());
        ship.parse_instruction("F11".to_string());

        assert_eq!(ship.manhattan_distance(), 286);
    }
}