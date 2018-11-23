use std::collections::HashMap;
use std::ops::Index;

use vm::Command;

struct Program {
    program: Vec<Command>,
    labels: HashMap<String, usize>,
}