use std::collections::HashMap;
use std::ops::Index;

use vm::Command;

pub struct Program {
    program: Vec<Command>,
    labels: HashMap<String, usize>,
}

impl Program {
    pub fn new(program: Vec<Command>, labels: HashMap<String, usize>) -> Self {
        Program {
            program,
            labels,
        }
    }

    pub fn get_address(&self, label: &str) -> usize {
        *self.labels.get(label).unwrap()
    }

    pub fn len(&self) -> usize {
        self.program.len()
    }
}

impl Index<usize> for Program {
    type Output = Command;
    
    fn index(&self, i: usize) -> &Command {
        &self.program[i]
    }
}