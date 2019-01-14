use common::{Sink, Source};
use program::Program;
use std::collections::HashMap;

use crate::moo::parse_program_from_string
use super::MooMachine;

impl MooMachine {
    fn get_program(&self) -> &Program {
        &self.program
    }

    fn get_registers(&self) -> &HashMap<u64, u64> {
        &self.registers
    }

    fn get_program_counter(&self) -> u64 {
        self.program_counter
    }

    fn get_inputs(&self) -> &Vec<Box<Source<u64>>> {
        &self.input
    }
    fn get_outputs(&self) -> &Vec<Box<Sink<u64>>> {
        &self.output
    } 
}

#[test]
fn load_float_test() {
    let program = parse_program_from_string("load 3f R1;");
    let machine = MooMachine::new(program: Program, Vec::new(), Vec::new());
    assert_eq!(machine.get_registers().get(1), 0.);
    machine.tick();
    assert_eq!(machine.get_registers().get(1), 1.)

}