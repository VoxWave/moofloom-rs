use std::collections::HashMap;
use std::ops::Index;

use crate::vm::Command;

#[derive(Debug, PartialEq)]
pub struct Program {
    program: Vec<Command>,
    labels: HashMap<String, u64>,
}

impl Program {
    pub fn new(program: Vec<Command>, labels: HashMap<String, u64>) -> Self {
        Program {
            program,
            labels,
        }
    }

    pub fn get_address(&self, label: &str) -> u64 {
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