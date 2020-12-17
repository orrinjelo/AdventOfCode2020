use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use std::collections::HashMap;
use regex::Regex;
use itertools::Itertools;
use crate::util::RetType;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

struct Mask {
    ones_mask: u64,
    zeros_mask: u64,
    exes_mask: Vec<u64>,

}

impl Mask {
    pub fn new() -> Mask {
        Mask {
            ones_mask: 0u64,
            zeros_mask: 0u64,
            exes_mask: Vec::new(),
        }
    }

    pub fn set(&mut self, val: String) {
        self.ones_mask = 0u64;
        self.zeros_mask = 0u64;
        for i in 0..val.len() {
            match val.as_bytes()[val.len()-1-i] as char {
                '1' => self.ones_mask += 2u64.pow(i as u32),
                '0' => self.zeros_mask += 2u64.pow(i as u32),
                _ => {},
            }
        }
    }

    pub fn set_v2(&mut self, val: String) {
        self.ones_mask = 0u64;
        self.zeros_mask = 0u64;
        self.exes_mask.clear();
        self.exes_mask.push(0);

        for i in 0..val.len() {
            match val.as_bytes()[val.len()-1-i] as char {
                '1' => self.ones_mask += 2u64.pow(i as u32),
                '0' => self.zeros_mask += 2u64.pow(i as u32),
                'X' => self.exes_mask.push(2u64.pow(i as u32)),
                _ => {}
            }
        }

    }

    pub fn apply(&mut self, val: u64) -> u64 {
        let mut v = val;
        v |= self.ones_mask;
        v &= !self.zeros_mask;
        v &= 2u64.pow(36) - 1;
        v
    }

    pub fn apply_v2(&mut self, val: u64) -> Vec<u64> {
        let base_addr = (val & self.zeros_mask) | self.ones_mask;

        trace!("baseaddr: {}", base_addr);
        trace!("ones: {}", self.ones_mask);
        trace!("zeros: {}", self.zeros_mask);
        trace!("zeros: {}", self.zeros_mask);

        let mut addresses: Vec<u64> = Vec::new();
        for i in 1..self.exes_mask.len()+1 {
            for j in self.exes_mask.iter().combinations(i) {
                trace!("Combinations: {:?}", j);
                let mut addendum = 0u64;
                for k in j.iter() {
                    addendum += *k;
                }
                trace!("Adding {} as an address.", addendum + base_addr);
                if !addresses.contains(&(base_addr + addendum)) {
                    addresses.push(base_addr + addendum);
                }
            }
        }
        addresses.sort();
        addresses
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Instruction {
    SetMask(String),
    SetMem(u64, u64),
}

fn parse_line(s: String) -> Instruction {
    let parts: Vec<&str> = s.split('=').collect();
    trace!("parts: {:?}", parts);
    if parts[0].trim() == "mask" {
        Instruction::SetMask(parts[1].trim().to_string())
    } else {
        let re = Regex::new(r"mem\[(\d+)\]").unwrap();
        let cap = re.captures(&parts[0]).unwrap();
        let left = cap[1].trim().parse::<u64>().unwrap();
        let right = parts[1].trim().parse::<u64>().unwrap();
        Instruction::SetMem(left, right)
    }
}

struct Computer {
    memory: HashMap<u64, u64>,
    mask: Mask,
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            memory: HashMap::new(),
            mask: Mask::new(),
        }
    }

    pub fn execute(&mut self, line: String) {
        match parse_line(line) {
            Instruction::SetMask(mask) => {
                self.mask.set(mask)
            },
            Instruction::SetMem(loc, val) => {
                if let Some(x) = self.memory.get_mut(&loc) {
                    *x = self.mask.apply(val);
                } else {
                    self.memory.insert(loc, self.mask.apply(val));
                }
            }
        };
    }

    pub fn execute_v2(&mut self, line: String) {
        match parse_line(line) {
            Instruction::SetMask(mask) => {
                self.mask.set_v2(mask)
            },
            Instruction::SetMem(loc, val) => {
                for address in self.mask.apply_v2(loc) {
                    trace!("Setmem on address: {}", address);
                    if let Some(x) = self.memory.get_mut(&address) {
                        trace!("Memory found!  Setting it to {}", val);
                        *x = val;
                    } else {
                        trace!("Memory not found!  Inserting {}: {}", address, val);
                        self.memory.insert(address, val);
                    }
                }
            }
        };
    }


    pub fn eval(&self) -> u64 {
        let mut sum = 0u64;
        for v in self.memory.values() {
            sum += *v;
        }
        sum
    }
}

/// Problem #14, part 1
pub fn problem_141(input: Vec<String>) -> RetType {
    let mut comp = Computer::new();
    for line in input {
        comp.execute(line);
    }
    RetType::U64(comp.eval())
}

/// Problem #14, part 2
pub fn problem_142(input: Vec<String>) -> RetType {
    let mut comp = Computer::new();
    for line in input {
        comp.execute_v2(line);
    }
    RetType::U64(comp.eval())
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
    fn test_mask() {
        init();

        let mut mask = Mask::new();

        mask.set("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string());

        assert_eq!(mask.ones_mask, 64);
        assert_eq!(mask.zeros_mask, 2);

        assert_eq!(mask.apply(11), 73);
        assert_eq!(mask.apply(101), 101);
        assert_eq!(mask.apply(0), 64);

        mask.set_v2("000000000000000000000000000000X1001X".to_string());

        assert_eq!(mask.ones_mask, 18);
        assert_eq!(mask.zeros_mask, 68719476684);
        assert_eq!(mask.exes_mask, vec![0, 1, 32]);

        let res = mask.apply_v2(42);
        assert_eq!(res, vec![26, 27, 58, 59]);
    }

    #[test]
    fn test_parse_line() {
        init();

        assert_eq!(
            parse_line("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string()), 
            Instruction::SetMask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string())
        );

        assert_eq!(
            parse_line("mem[8] = 11".to_string()),
            Instruction::SetMem(8, 11)
        );
    }

    #[test]
    fn test_computer() {
        init();

        let mut comp = Computer::new();

        comp.execute("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string());
        comp.execute("mem[8] = 11".to_string());
        comp.execute("mem[7] = 101".to_string());
        comp.execute("mem[8] = 0".to_string());

        trace!("memory: {:?}", comp.memory);

        assert_eq!(comp.memory.get(&7).unwrap(), &101u64);
        assert_eq!(comp.memory.get(&8).unwrap(), &64u64);

        assert_eq!(comp.eval(), 165u64);
    }

    #[test]
    fn test_computer_v2() {
        init();

        let mut comp = Computer::new();

        comp.execute_v2("mask = 000000000000000000000000000000X1001X".to_string());
        comp.execute_v2("mem[42] = 100".to_string());
        comp.execute_v2("mask = 00000000000000000000000000000000X0XX".to_string());
        comp.execute_v2("mem[26] = 1".to_string());

        trace!("memoryv2: {:?}", comp.memory);

        // Set to 100:
        // 000000000000000000000000000000111010  (decimal 58)
        // 000000000000000000000000000000111011  (decimal 59)
        // Set to 1:
        // 000000000000000000000000000000010000  (decimal 16)
        // 000000000000000000000000000000010001  (decimal 17)
        // 000000000000000000000000000000010010  (decimal 18)
        // 000000000000000000000000000000010011  (decimal 19)
        // 000000000000000000000000000000011000  (decimal 24)
        // 000000000000000000000000000000011001  (decimal 25)
        // 000000000000000000000000000000011010  (decimal 26)
        // 000000000000000000000000000000011011  (decimal 27)

        assert_eq!(comp.memory.get(&58).unwrap(), &100u64);
        assert_eq!(comp.memory.get(&59).unwrap(), &100u64);
        assert_eq!(comp.memory.get(&16).unwrap(), &1u64);
        assert_eq!(comp.memory.get(&19).unwrap(), &1u64);
        assert_eq!(comp.memory.get(&24).unwrap(), &1u64);
        assert_eq!(comp.memory.get(&26).unwrap(), &1u64);
        assert_eq!(comp.memory.get(&27).unwrap(), &1u64);

        assert_eq!(comp.eval(), 208u64);
    }
}