use std::io::Read;

use Program;
use vm::Command;

pub fn parse_program<R: Read>(r: R) -> Program {
    let mut source = String::new();
    r.read_to_string(&mut source).expect("IO error! Reading source file into string failed");

    for s in source.split(';') {
        let c: Vec<_> = s.split(' ').collect();
        match &*c[0].to_lowercase() {
            "fadd" => {},
            "fsub" => {},
            "fmul" => {},
            "fdiv" => {},
            "load" => {},
            _ => unimplemented!("Command does not exist."),
        }
    }
}