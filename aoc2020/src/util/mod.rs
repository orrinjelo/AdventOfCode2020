use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub mod macros;

/// Utility function to read lines from a file
/// Opens and reads a file, returns a vector of strings 
///  wrapped in a Result
/// 
/// # Arguments
/// filename - String filename path
///
/// # Returns
/// Result of a Vector of Strings
fn lines_from_file(filename: String) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

/// Load strings from a file
///
/// # Arguments
/// filename - String filename path
///
/// # Returns
/// A Vector of strings
pub fn load_file(filename: String) -> Vec<String> {
    lines_from_file(filename)
        .expect("Could not read from file")
} 