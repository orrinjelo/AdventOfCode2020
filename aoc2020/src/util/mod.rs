use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::fmt;
// use gif;
// use std::borrow::Cow;
// use std::convert::TryInto;

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

#[allow(dead_code)]
pub fn flatten<T>(nested: Vec<Vec<T>>) -> Vec<T> {
    nested.into_iter().flatten().collect()
}

// #[allow(dead_code)]
// fn vec_to_array<T>(v: Vec<T>) -> [T; 64] where T: Copy {
//     let slice = v.as_slice();
//     let array: [T; 32] = match slice.try_into() {
//         Ok(ba) => ba,
//         Err(_) => panic!("Expected a Vec of length {} but it was {}", 32, v.len()),
//     };
//     array
// }

// pub struct GifImage<'a> {
//     color_map: &'a [u8],
//     width: u16,
//     height: u16,
//     encoder: gif::Encoder,
// }

// impl GifImage<'_> {
//     pub fn new(filename: String, w: u16, h: u16) -> GifImage<'static> {
//         let im = File::create(filename).unwrap();
//         let enc = gif::Encoder::new(&mut im, w, h, &[0xFF, 0xFF, 0xFF, 0, 0, 0]).unwrap();
//         GifImage {
//             color_map: &[0xFF, 0xFF, 0xFF, 0, 0, 0],
//             width: w,
//             height: h,
//             encoder: enc
//         }
//     }

//     pub fn write_frame(&mut self, f: Vec<u8>) {
//         let mut frame = gif::Frame::default();
//         frame.width = self.width;
//         frame.height = self.height;
//         // const frame_size: u16 = self.width*self.height;
//         let r: Result<[u8; 96], _> = f.try_into();
//         frame.buffer = Cow::Borrowed(r.unwrap());
//         self.encoder.write_frame(&frame).unwrap();
//     }
// }

#[derive(Clone, PartialEq)]
pub enum RetType {
    U32(u32),
    I32(i32),
    U64(u64),
    U128(u128)
}

impl fmt::Debug for RetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RetType::U32(x) => write!(f, "{:?}", x),
            RetType::I32(x) => write!(f, "{:?}", x),
            RetType::U64(x) => write!(f, "{:?}", x),
            RetType::U128(x) => write!(f, "{:?}", x),
        }
        
    }
}

impl fmt::Display for RetType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RetType::U32(x) => write!(f, "{}", x),
            RetType::I32(x) => write!(f, "{}", x),
            RetType::U64(x) => write!(f, "{}", x),
            RetType::U128(x) => write!(f, "{}", x),
        }
        
    }
}