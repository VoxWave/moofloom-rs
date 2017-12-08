use std::io::Read;

use Program;
use vm::Command;

pub fn parse_program<R: Read>(r: R) -> Result<Program, MooParseError> {
    let mut source = String::new();
    r.read_to_string(&mut source).expect("IO error! Reading source file into string failed");
    let mut program = Vec::new();

    for instruction in source.split(';') {
        let params: Vec<_> = instruction.split(' ').collect();
        match &*params[0].to_lowercase() {
            "fadd" => {
                
            },
            "fsub" => {},
            "fmul" => {},
            "fdiv" => {},
            "load" => {},
            _ => return Err(MooParseError::CommandNotFound(instruction.to_string())),
        }
    }
}

enum MooParseError {
    CommandNotFound(String), IOError(::std::io::Error), InsufficientParamAmount
}