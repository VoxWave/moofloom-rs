use std::collections::HashMap;
use std::mem::transmute;

#[cfg(test)]
mod vm_test;

type Program = Vec<Command>;
const SIGNALING_NAN_MASK: u64 = 0x7FF8000000000000;

pub struct Machine {
    program: Program,
    registers: HashMap<u64,u64>,
    program_counter: u64,
}
impl Machine {
    pub fn new(program: Program) -> Self {
        Machine{
            program: program,
            registers: HashMap::new(),
            program_counter: 0,
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
            FAdd(a, b, into) => self.float_add(a, b, into),
            FSub(a, b, into) => self.float_sub(a, b, into), 
            FMul(a, b, into) => self.float_mul(a, b, into),
            FDiv(a, b, into) => self.float_div(a, b, into),
            Load(what, into) => self.load(what, into),
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
                fConstant(what) => {
                    self.store_float(what, into);
                }
                _ => unimplemented!("this type of loading is unimplemented"),
            }
        } else {
            panic!("Load target was not a register.");
        }
    }

    fn float_add(&mut self, a: Param, b: Param, into: Param) {
        use self::Param::*;
        if let Register(into) = into {
            let a = self.get_float(a);
            let b = self.get_float(b);
            self.store_float(a+b, into);
        } else {
            panic!("The target parameter of float addition wasn't a register.");
        }
    }

    fn float_sub(&mut self, a: Param, b: Param, into: Param) {
        use self::Param::*;
        if let Register(into) = into {
            let a = self.get_float(a);
            let b = self.get_float(b);
            self.store_float(a-b, into);
        } else {
            panic!("The target parameter of float addition wasn't a register.");
        }
    }

    fn float_mul(&mut self, a: Param, b: Param, into: Param) {
        use self::Param::*;
        if let Register(into) = into {
            let a = self.get_float(a);
            let b = self.get_float(b);
            self.store_float(a*b, into);
        } else {
            panic!("The target parameter of float addition wasn't a register.");
        }
    }

    fn float_div(&mut self, a: Param, b: Param, into: Param) {
        use self::Param::*;
        if let Register(into) = into {
            let a = self.get_float(a);
            let b = self.get_float(b);
            if b == 0. {
                panic!("Float divide by zero");
            }
            self.store_float(a/b, into);
        } else {
            panic!("The target parameter of float division wasn't a register.");
        }
    }

    fn store_float(&mut self, what:f64, into: u64) {
        let what: u64 = unsafe {
            transmute(what)
        };
        self.registers.insert(into, what);
    }

    fn get_float(&self, param: Param) -> f64 {
        use self::Param::*;
        match param {
            Register(register) => self.load_float_from_register(register),
            fConstant(float) => float,
            _ => panic!("a parameter could not be used as a float"),
        }
    }

    fn load_float_from_register(&self, register: u64) -> f64 {
        let val = self.registers.get(&register).unwrap_or(&0);
        if val & SIGNALING_NAN_MASK == SIGNALING_NAN_MASK {
            panic!("tried to load signaling NaN");
        }
        let val: f64 = unsafe {
            transmute(val)
        };
        if val.is_nan() {
            panic!("tried to load nan");
        } else if val.is_infinite() {
            panic!("tried to load infinity");
        }
        val
    }
}
///General order of the parameters is (what, where)
#[derive(Clone, Copy)]
pub enum Command {
    FAdd(Param, Param, Param),
    FSub(Param, Param, Param), 
    FMul(Param, Param, Param), 
    FDiv(Param, Param, Param),
    Load(Param, Param),
}
#[derive(Clone, Copy)]
pub enum Param {
    Register(u64), fConstant(f64),
}

pub struct MooParser {

}