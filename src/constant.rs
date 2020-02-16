use crate::instruction::Instruction;

pub enum Constant {
    None,
    Integer(i64),
    Real(f64),
    Char(char),
    Boolean(bool),
    Function { arity: u8, func_object: FuncObject },
}

pub enum FuncObject {
    CodeObject {
        code: Vec<Instruction>,
        const_table: Vec<Constant>,
    },
    NativeFunc {
        function: Box<Fn(Vec<Constant>) -> Constant>,
    },
}
