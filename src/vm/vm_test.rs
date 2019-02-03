use common::{Sink, Source};
use program::Program;
use std::collections::HashMap;
use std::mem::transmute;

use super::{Command, MooMachine, Param};
use crate::moo::parse_program_from_string;
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
fn fadd_test() {
    let program = Program::new(
        vec![Command::FAdd(
            Param::FConstant(1.),
            Param::FConstant(2.),
            Param::Register(0),
        )],
        HashMap::new(),
    );
    let mut machine = MooMachine::new(program, Vec::new(), Vec::new());
    machine.tick();
    assert_eq!(
        *(machine.get_registers().get(&0).unwrap()), 
        unsafe{ transmute(3f64) },
    );
}

#[test]
fn load_float_test() {
    let program = parse_program_from_string("load 3f R1;").unwrap();
    let mut machine = MooMachine::new(program, Vec::new(), Vec::new());
    assert_eq!(
        machine.get_registers().get(&1), 
        None,    
    );
    machine.tick();
    assert_eq!(
        *machine.get_registers().get(&1).unwrap(), 
        unsafe{ transmute(3.) }
    )
}