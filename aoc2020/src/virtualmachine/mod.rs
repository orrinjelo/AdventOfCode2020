#[allow(dead_code)]
use std::str::FromStr;
use std::num::ParseIntError;
use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum InstructionCode {
    NOP,
    ACC,
    JMP,
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    operator: InstructionCode,
    operand: i32
}

impl Instruction {
    pub fn parse_string(input_str: String) -> Instruction {
        let parts: Vec<String> = input_str.split(' ').map(|x| x.to_string()).collect(); 
        let op = match &parts[0][..] {
            "nop" => InstructionCode::NOP,
            "acc" => InstructionCode::ACC,
            "jmp" => InstructionCode::JMP,
            _ => InstructionCode::NOP,
        };

        let opa = parts[1].parse::<i32>();
        match opa {
            Ok(a) => {
                Instruction {
                    operator: op,
                    operand: a
                }
            },
            Err(_) => {
                Instruction {
                    operator: InstructionCode::NOP,
                    operand: 0,
                }
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<String> = s.split(' ').map(|x| x.to_string()).collect(); 
        let op = match &parts[0][..] {
            "nop" => InstructionCode::NOP,
            "acc" => InstructionCode::ACC,
            "jmp" => InstructionCode::JMP,
            _ => InstructionCode::NOP,
        };

        let opa = parts[1].parse::<i32>()?;
        Ok(Instruction {
            operator: op,
            operand: opa,
        })
    }
}

pub struct Vm {
    code: Vec<Instruction>,
    pc: i32,
    accumulator: i32,
    history: Vec<i32>,
}

impl Vm {
    #[allow(dead_code)]
    pub fn new(lines: Vec<String>) -> Vm {
        let code = lines.iter().map(|x| Instruction::parse_string(x.clone())).collect();

        Vm {
            code: code,
            pc: 0,
            accumulator: 0,
            history: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn execute_once(&mut self) {
        let inst = &self.code[self.pc.clone() as usize];

        trace!("{:?}", inst);
        trace!("Old PC: {}", self.pc);

        self.history.push(self.pc);

        match inst.operator {
            InstructionCode::NOP => {
                self.pc += 1;
            },
            InstructionCode::ACC => {
                self.accumulator += inst.operand;
                self.pc += 1;
            },
            InstructionCode::JMP => {
                self.pc += inst.operand;
            }
        }

        trace!("New PC: {}", self.pc);
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.accumulator = 0;
        self.history.clear();
    }

    #[allow(dead_code)]
    pub fn execute_until_repeat(&mut self) {
        while !self.history.contains(&self.pc) {
            self.execute_once();
        }
    }

    #[allow(dead_code)]
    pub fn execute_gamegirl(&mut self) {
        let backup = self.code.clone();
        let mut investigator: i32 = -1;

        while self.pc < self.code.len() as i32 {
            self.execute_once();

            trace!("History: {:?}", self.history);

            if self.history.contains(&self.pc) {
                // Try auto correct
                self.code.copy_from_slice(&backup[..]);
                self.reset();
                'investigation: loop {
                    investigator += 1;
                    if investigator >= self.code.len() as i32 {
                        break 'investigation;
                    }
                    match self.code[investigator as usize].operator {
                        InstructionCode::JMP => {
                            trace!("Trying to auto-correct {}", investigator);
                            self.code[investigator as usize].operator = InstructionCode::NOP;
                            break 'investigation;
                        },
                        InstructionCode::NOP => {
                            trace!("Trying to auto-correct {}", investigator);
                            self.code[investigator as usize].operator = InstructionCode::JMP;
                            break 'investigation;
                        },
                        _ => {}
                    }
                }

            }
        }
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    pub fn get_acc(&self) -> i32 {
        self.accumulator
    }

    #[allow(dead_code)]
    pub fn get_history(&self) -> Vec<i32> {
        self.history.clone()
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
    fn test_instruction_parsing() {
        init();

        let inst1 = Instruction::parse_string("nop +0".to_string());
        let inst2 = Instruction::parse_string("jmp +4".to_string());
        let inst3 = Instruction::parse_string("acc -99".to_string());

        assert_eq!(inst1.operator, InstructionCode::NOP);
        assert_eq!(inst2.operator, InstructionCode::JMP);
        assert_eq!(inst3.operator, InstructionCode::ACC);
        assert_eq!(inst1.operand, 0);
        assert_eq!(inst2.operand, 4);
        assert_eq!(inst3.operand, -99);
    }

    #[test]
    fn test_vm_creation_and_run() {
        init();

        let input = vec![
            "nop +0".to_string(),
            "acc +1".to_string(),
            "jmp +4".to_string(),
            "acc +3".to_string(),
            "jmp -3".to_string(),
            "acc -99".to_string(),
            "acc +1".to_string(),
            "jmp -4".to_string(),
            "acc +6".to_string(),
        ];

        let mut vm = Vm::new(input);
        // vm.execute_once();
        vm.execute_until_repeat();

        assert_eq!(vm.get_acc(), 5);
    }

    #[test]
    fn test_vm_autocorrect() {
        init();

        let input = vec![
            "nop +0".to_string(),
            "acc +1".to_string(),
            "jmp +4".to_string(),
            "acc +3".to_string(),
            "jmp -3".to_string(),
            "acc -99".to_string(),
            "acc +1".to_string(),
            "jmp -4".to_string(),
            "acc +6".to_string(),
        ];

        let mut vm = Vm::new(input);
        vm.execute_gamegirl();

        assert_eq!(vm.get_acc(), 8);
    }
}