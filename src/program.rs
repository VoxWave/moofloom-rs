use std::collections::HashMap;
use std::ops::Index;

use vm::Command;

pub struct Program {
    program: Vec<Command>,
    labels: HashMap<String, usize>,
}

impl Index<usize> for Program {
    type Output = Command;
    
    fn index(&self, i: usize) -> &Command {
        &self.program[i]
    }
}