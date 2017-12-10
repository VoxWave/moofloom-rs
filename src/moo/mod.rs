use std::io::Read;

use Program;
use vm::{Command, Param};

pub fn parse_program<R: Read>(r: R) -> Result<Program, MooParseError> {
    let mut source = String::new();
    r.read_to_string(&mut source).expect("IO error! Reading source file into string failed");
    let mut program = Vec::new();

    for instruction in source.split(';') {
        let params: Vec<_> = instruction.split(' ').collect();
        match &*params[0].to_lowercase().trim() {
            i @ "fadd" | i @ "fsub" | i @ "fmul" | i @ "fdiv" => {
                if let (p1, p2, p3 @ Param::Register(_)) = parse_three_params(params)? {
                    match i {
                        "fadd" => program.push(Command::FAdd(p1, p2, p3)),
                        "fsub" => program.push(Command::FSub(p1, p2, p3)),
                        "fmul" => program.push(Command::FMul(p1, p2, p3)),
                        "fdiv" => program.push(Command::FDiv(p1, p2, p3)),
                    }
                } else {
                    return Err(MooParseError::InvalidSyntax(format!("Third parameter in \"{}\" should be a register", instruction.to_string())));
                }
            },
            "load" => {
                return Err(MooParseError::CommandNotFound);
            },
            _ => return Err(MooParseError::CommandNotFound(instruction.to_string())),
        }
    }
}

pub fn parse_three_params(params: &Vec<str>) -> Result<(Param, Param, Param), MooParseError> {
    Err(MooParseError::CommandNotFound)
}

enum MooParseError {
    CommandNotFound(String), 
    IOError(::std::io::Error), 
    InsufficientParamAmount,
    InvalidParam(String),
    InvalidSyntax(String),
}