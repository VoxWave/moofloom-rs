use std::io::{self, Read};
use std::collections::HashMap;

use vm::{Command, Param};
use program::Program;

#[cfg(test)]
mod parser_test;

pub fn parse_program<R: Read>(mut r: R) -> Result<Program, MooParseError> {
    let mut source = String::new();
    r.read_to_string(&mut source)?;
    parse_program_from_string(&source)
}

pub fn parse_program_from_string(source: &str) -> Result<Program, MooParseError> {
    let mut instructions = Vec::new();
    let mut labels = HashMap::new();
    let mut line_count = 0;

    for line in source
        .split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {   
        let label_and_instruction: Vec<_> = line.split(':').map(|s| s.trim()).collect();
        match label_and_instruction.len() {
            1 => {
                instructions.push(parse_instruction(label_and_instruction[0])?);
            },
            2 => {
                labels.insert(label_and_instruction[0].to_string(), line_count);
                instructions.push(parse_instruction(label_and_instruction[1])?);
            },
            _ => return Err(MooParseError::InvalidLineStructure(line.to_string())),
        }
        line_count += 1;
    }
    Ok(Program::new(instructions, labels))
}

fn parse_instruction(instruction: &str) -> Result<Command, MooParseError> {
    let params: Vec<_> = instruction.split(' ').collect();
    match &*params[0].to_lowercase().trim() {
        i @ "fadd" | i @ "fsub" | i @ "fmul" | i @ "fdiv" => {
            if let (p1, p2, p3 @ Param::Register(_)) = parse_three_params(&params)? {
                match i {
                    "fadd" => Ok(Command::FAdd(p1, p2, p3)),
                    "fsub" => Ok(Command::FSub(p1, p2, p3)),
                    "fmul" => Ok(Command::FMul(p1, p2, p3)),
                    "fdiv" => Ok(Command::FDiv(p1, p2, p3)),
                    _ => unreachable!(),
                }
            } else {
                Err(MooParseError::InvalidSyntax(format!(
                    "Third parameter in \"{}\" should be a register",
                    instruction.to_string()
                )))
            }
        },
        "load" => {
            match parse_two_params(&params)? {
                (p1, p2 @ Param::Register(_)) => {
                    Ok(Command::Load(p1, p2))
                }
                _ => Err(MooParseError::InvalidParam(instruction.to_string())),
            }
        },
        "jump" => {
            // We just take whatever is after the jump and trim it to make it a label.
            Ok(Command::Jump(instruction[4..instruction.len()].trim().to_string()))
        },
        _ => Err(MooParseError::CommandNotFound(instruction.to_string())),
    }
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
        param[1..]
            .parse()
            .map(Param::Register)
            .map_err(|_| MooParseError::InvalidParam(param.to_string()))
    } else if param.ends_with('f') {
        param[..(param.len() - 1)]
            .parse()
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
    InvalidLineStructure(String),
}

impl From<io::Error> for MooParseError {
    fn from(err: io::Error) -> Self {
        MooParseError::IOError(err)
    }
}
