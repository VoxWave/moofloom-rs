use std::collections::HashMap;
use std::ops::{Add, Sub, Mul};

use common::{Sink, Source};

use Program;

#[cfg(test)]
mod vm_test;

pub struct MooMachine {
    program: Program,
    registers: HashMap<u64,u64>,
    program_counter: u64,
    input: Vec<Box<Source<u64>>>,
    output: Vec<Box<Sink<u64>>>,
}
impl MooMachine {
    pub fn new(program: Program, input: Vec<Box<Source<u64>>>, output: Vec<Box<Sink<u64>>>) -> Self {
        MooMachine {
            program: program,
            registers: HashMap::new(),
            program_counter: 0,
            input,
            output,
        }
    }

    pub fn tick(&mut self) {
        use self::Command::*;

        let pc = self.program_counter as usize;
        self.program_counter += 1;

        if pc >= self.program.len() {
            panic!("program counter overflowed");
        }

        match self.program[pc] {
            FAdd(a, b, into) => self.float_op(f64::add, "addition", a, b, into),
            FSub(a, b, into) => self.float_op(f64::sub, "substraction", a, b, into),
            FMul(a, b, into) => self.float_op(f64::mul, "multiplication", a, b, into),
            FDiv(a, b, into) => self.float_op(|a, b| if b == 0. {
                panic!("Float division by a zero");
            } else {
                a / b
            }, "division", a, b, into),
            UAdd(a, b, into) => self.unsigned_integer_op(u64::add, "addition", a, b, into),
            USub(a, b, into) => self.unsigned_integer_op(u64::sub, "substraction", a, b, into),
            UMul(a, b, into) => self.unsigned_integer_op(u64::mul, "multiplication", a, b, into),
            UDiv(a, b, into) => self.unsigned_integer_op(|a, b| if b == 0 {
                panic!("Unsigned integer division by a zero");
            } else {
                a / b
            }, "division", a, b, into),
            IAdd(a, b, into) => self.signed_integer_op(i64::add, "addition", a, b, into),
            ISub(a, b, into) => self.signed_integer_op(i64::sub, "substraction", a, b, into),
            IMul(a, b, into) => self.signed_integer_op(i64::mul, "multiplication", a, b, into),
            IDiv(a, b, into) => self.signed_integer_op(|a, b| if b == 0 {
                panic!("Unsigned integer division by a zero");
            } else {
                a / b
            }, "division", a, b, into),
            Load(what, into) => self.load(what, into),
            _ => {},
        }
    }

    fn load(&mut self, what: Param, into: Param) {
        use self::Param::*;
        if let Register(into) = into {
            match what {
                Register(what) => {
                    let a = *self.registers.get(&what).unwrap_or(&0);
                    self.registers.insert(into, a);
                },
                FConstant(what) => {
                    self.store_float(what, into);
                }
                _ => unimplemented!(),
            }
        } else {
            panic!("Load target was not a register.");
        }
    }

    fn float_op<F>(&mut self, op: F, op_name: &str, a: Param, b: Param, into: Param)
        where F: Fn(f64, f64) -> f64
    {
        let a = self.get_float(a);
        let b = self.get_float(b);
        self.store_float(op(a,b), into)
    }

    // fn unsigned_integer_op<F>(&mut self, op: F, op_name: &str, a: Param, b: Param, into: Param)
    //     where F: Fn(u64, u64) -> u64
    // {
    //     use self::Param::*;
    //     if let Register()
    // }
    
    // fn signed_integer_op<F>(&mut self, op: F, op_name: &str, a: Param, b: Param, into: Param)
    //     where F: Fn(i64, i64) -> i64
    // {
        
    // }

    fn store_float(&mut self, what:f64, into: Param) {
        use self::Param::*;
        match into {
            Register(into) => {
                self.registers.insert(into, what.to_bits());
            },
            Output(into) => self.output.get(into as usize).unwrap().put(what.to_bits()),
        }
    }

    fn get_float(&self, param: Param) -> f64 {
        use self::Param::*;
        match param {
            Register(register) => self.load_float_from_register(register),
            FConstant(float) => float,
            _ => unimplemented!(),
        }
    }

    fn load_float_from_register(&self, register: u64) -> f64 {
        let val = self.registers.get(&register).unwrap_or(&0);
        let val = f64::from_bits(*val);
        if val.is_nan() {
            panic!("tried to load nan");
        } else if val.is_infinite() {
            panic!("tried to load infinity");
        }
        val
    }
}
///General order of the parameters is (what, where)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    IAdd(Param, Param, Param),
    ISub(Param, Param, Param),
    IMul(Param, Param, Param),
    IDiv(Param, Param, Param),
    UAdd(Param, Param, Param),
    USub(Param, Param, Param),
    UMul(Param, Param, Param),
    UDiv(Param, Param, Param),
    FAdd(Param, Param, Param),
    FSub(Param, Param, Param),
    FMul(Param, Param, Param),
    FDiv(Param, Param, Param),
    Load(Param, Param),
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Param {
    Register(u64), FConstant(f64), IConstant(i64), UConstant(u64), Input(u64), Output(u64)
}