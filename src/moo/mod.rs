use std::io::{self, Read};

use Program;
use vm::{Command, Param};

#[cfg(test)]
mod parser_test;

pub fn parse_program<R: Read>(mut r: R) -> Result<Program, MooParseError> {
    let mut source = String::new();
    r.read_to_string(&mut source)?;
    parse_program_from_string(source)
}

pub fn parse_program_from_string(source: String) -> Result<Program, MooParseError> {
    let mut program = Vec::new();

    for instruction in source.split(';') {
        let params: Vec<_> = instruction.trim().split(' ').collect();
        match &*params[0].to_lowercase().trim() {
            i @ "fadd" | i @ "fsub" | i @ "fmul" | i @ "fdiv" => {
                if let (p1, p2, p3 @ Param::Register(_)) = parse_three_params(&params)? {
                    match i {
                        "fadd" => program.push(Command::FAdd(p1, p2, p3)),
                        "fsub" => program.push(Command::FSub(p1, p2, p3)),
                        "fmul" => program.push(Command::FMul(p1, p2, p3)),
                        "fdiv" => program.push(Command::FDiv(p1, p2, p3)),
                        _ => unreachable!(),
                    }
                } else {
                    return Err(MooParseError::InvalidSyntax(format!("Third parameter in \"{}\" should be a register", instruction.to_string())));
                }
            },
            "load" => {
                if let (p1, p2 @ Param::Register(_)) = parse_two_params(&params)? {
                    program.push(Command::Load(p1, p2));
                }
            },
            _ => return Err(MooParseError::CommandNotFound(instruction.to_string())),
        }
    };
    Ok(program)
}

pub fn parse_three_params(params: &Vec<&str>) -> Result<(Param, Param, Param), MooParseError> {
    if params.len() == 4 {
        let param1 = parse_param(params[1])?;
        let param2 = parse_param(params[2])?;
        let param3 = parse_param(params[3])?;
        Ok((param1, param2, param3))
    } else {
        Err(MooParseError::InvalidParamAmount)
    }
}

pub fn parse_two_params(params: &Vec<&str>) -> Result<(Param, Param), MooParseError> {
    if params.len() == 3 {
        let param1 = parse_param(params[1])?;
        let param2 = parse_param(params[2])?;
        Ok((param1, param2))
    } else {
        Err(MooParseError::InvalidParamAmount)
    }
}

pub fn parse_param(param: &str) -> Result<Param, MooParseError> {
    let param = param.trim().to_lowercase();
    if param.starts_with('r') {
        param[1..].parse()
            .map(Param::Register)
            .map_err(|_| MooParseError::InvalidParam(param.to_string())) 
    } else if param.ends_with('f') {
        param[..(param.len()-1)].parse()
            .map(Param::FConstant)
            .map_err(|_| MooParseError::InvalidParam(param.to_string()))
    } else {
        Err(MooParseError::InvalidParam(param.to_string()))
    }
}

#[derive(Debug)]
pub enum MooParseError {
    CommandNotFound(String), 
    IOError(::std::io::Error), 
    InvalidParamAmount,
    InvalidParam(String),
    InvalidSyntax(String),
}


impl From<io::Error> for MooParseError {
    fn from(err: io::Error) -> Self {
        MooParseError::IOError(err)
    }
}