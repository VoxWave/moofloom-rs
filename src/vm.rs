pub struct Machine {
    code: Vec<Command>
}

enum Command {
    Add(MemoryType, MemoryType, MemoryType), Mul, Div,
}

enum MemoryType {
    Register(usize),
}