use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Cmp(Ordering),
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOp {
    Negate,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Opcode {
    Push(u32),
    Pop,
    Load(u32),
    StoreGlobal(String),
    LoadGlobal(String),
    Call(u32),
    Jmp(u32),
    PopJmpIfFalse(u32),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
}
