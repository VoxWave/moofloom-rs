use common::{Sink, Source};
use program::Program;
use std::collections::HashMap;
use std::mem::transmute;

use super::{Command, MooMachine, Param};
use crate::moo::parse_program_from_string;
// impl MooMachine {
//     fn get_program(&self) -> &Program {
//         &self.program
//     }

//     fn get_registers(&self) -> &HashMap<u64, u64> {
//         &self.registers
//     }

//     fn get_program_counter(&self) -> u64 {
//         self.program_counter
//     }

//     fn get_inputs(&self) -> &Vec<Box<Source<u64>>> {
//         &self.input
//     }
//     fn get_outputs(&self) -> &Vec<Box<Sink<u64>>> {
//         &self.output
//     } 
// }

#[test]
fn fadd_test() {
    let program = Program::new(
        vec![Command::FAdd(
            Param::FConstant(1.),
            Param::FConstant(2.),
            Param::Register(0),
        )],
        HashMap::new(),
    );
    let mut machine = MooMachine::new(program, Vec::new(), Vec::new());
    machine.tick();
    assert_eq!(
        *(machine.registers.get(&0).unwrap()), 
        3f64.to_bits(),
    );
}

#[test]
fn load_float_test() {
    let program = parse_program_from_string("load 3f R1;").unwrap();
    let mut machine = MooMachine::new(program, Vec::new(), Vec::new());
    assert_eq!(
        machine.registers.get(&1), 
        None,    
    );
    machine.tick();
    assert_eq!(
        *machine.registers.get(&1).unwrap(), 
        3f64.to_bits(),
    )
}

#[test]
fn jump_test() {
    let source = "label1:Jump label3\n; label2 : jump label1;\n label3: jump label2;";
    let program = parse_program_from_string(source).unwrap();
    let mut machine = MooMachine::new(program, Vec::new(), Vec::new());

    assert_eq!(machine.program_counter, 0);
    machine.tick();
    assert_eq!(machine.program_counter, 2);
    machine.tick();
    assert_eq!(machine.program_counter, 1);
    machine.tick();
    assert_eq!(machine.program_counter, 0);
}

#[test]
fn arithmetic_operators_and_load_for_floats_test() {
    let source = r#"fadd 1f 2f R0;
    fsub 1f 2f R0
;   fmul 1f 2f R0;
fdiv 1f 2f R0;
load 1f R0;"#;
    let program = parse_program_from_string(source).unwrap();
    let mut machine = MooMachine::new(program, Vec::new(), Vec::new());

    assert_eq!(machine.registers.get(&0), None);
    machine.tick();
    assert_eq!(f64::from_bits(*machine.registers.get(&0).unwrap()), 1. + 2.);
    machine.tick();
    assert_eq!(f64::from_bits(*machine.registers.get(&0).unwrap()), 1. - 2.);
    machine.tick();
    assert_eq!(f64::from_bits(*machine.registers.get(&0).unwrap()), 1. * 2.);
    machine.tick();
    assert_eq!(f64::from_bits(*machine.registers.get(&0).unwrap()), 1. / 2.);
    machine.tick();
    assert_eq!(f64::from_bits(*machine.registers.get(&0).unwrap()), 1.);
}

#[test]
fn arithmetic_operators_and_load_for_signed_integers_test() {
    let source = r#"iadd 1i 2i R0;
    isub 1i 2i R0
;   imul 1i 2i R0;
idiv 1i 2i R0;
load 1i R0;"#;
    let program = parse_program_from_string(source).unwrap();
    let mut machine = MooMachine::new(program, Vec::new(), Vec::new());

    assert_eq!(machine.registers.get(&0), None);
    machine.tick();
    assert_eq!(unsafe{transmute::<_, i64>(*machine.registers.get(&0).unwrap())}, 1 + 2);
    machine.tick();
    assert_eq!(unsafe{transmute::<_, i64>(*machine.registers.get(&0).unwrap())}, 1 - 2);
    machine.tick();
    assert_eq!(unsafe{transmute::<_, i64>(*machine.registers.get(&0).unwrap())}, 1 * 2);
    machine.tick();
    assert_eq!(unsafe{transmute::<_, i64>(*machine.registers.get(&0).unwrap())}, 1 / 2);
    machine.tick();
    assert_eq!(unsafe{transmute::<_, i64>(*machine.registers.get(&0).unwrap())}, 1);
}

#[test]
fn arithmetic_operators_and_load_for_unsigned_test() {
    let source = r#"uadd 1u 2u R0;
    usub 1u 2u R0
;   umul 1u 2u R0;
udiv 1u 2u R0;
load 1u R0;"#;
    let program = parse_program_from_string(source).unwrap();
    let mut machine = MooMachine::new(program, Vec::new(), Vec::new());

    assert_eq!(machine.registers.get(&0), None);
    machine.tick();
    assert_eq!(*machine.registers.get(&0).unwrap(), 1 + 2);
    machine.tick();
    //TODO: I need to define what moofloom does when underflow and overflows happen. 
    //Currently it does what rust does but that's inconvenient and inconsistent.
    assert_eq!(*machine.registers.get(&0).unwrap(), 1 - 2);
    machine.tick();
    assert_eq!(*machine.registers.get(&0).unwrap(), 1 * 2);
    machine.tick();
    assert_eq!(*machine.registers.get(&0).unwrap(), 1 / 2);
    machine.tick();
    assert_eq!(*machine.registers.get(&0).unwrap(), 1);
}