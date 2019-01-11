use common::{Sink, Source};
use program::Program;
use std::collections::HashMap;

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