pub struct Machine {
    code: Vec<Command>,
    registers: Vec<u64>,
    program_counter: u64,
}

enum Command {
    fAdd(Param, Param, Param),
    fSub(Param, Param, Param), 
    fMul(Param, Param, Param), 
    fDiv(Param, Param, Param),
    Load(Param, Param),
}

enum Param {
    Register(usize), fConstant(f64),
}