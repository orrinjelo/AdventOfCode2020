use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::virtualmachine;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

/// Problem #08, part 1
/// Run your copy of the boot code. Immediately before any instruction is 
///  executed a second time, what value is in the accumulator?
pub fn problem_081(input: Vec<String>) -> i32 {
    let mut vm = virtualmachine::Vm::new(input);
    vm.execute_until_repeat();
    vm.get_acc()
}

/// Problem #08, part 2
/// Fix the program so that it terminates normally by changing exactly one
///  jmp (to nop) or nop (to jmp). What is the value of the accumulator after 
///  the program terminates?
pub fn problem_082(input: Vec<String>) -> i32 {
    let mut vm = virtualmachine::Vm::new(input);
    vm.execute_gamegirl();
    vm.get_acc()
}
