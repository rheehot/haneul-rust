use std::fmt;

use crate::constant::Constant;
use crate::instruction::Instruction;

#[derive(Clone)]
pub enum FuncObject {
    CodeObject {
        code: Vec<Instruction>,
        const_table: Vec<Constant>,
    },
    NativeFunc {
        function: Box<fn(Vec<Constant>) -> Constant>,
    },
}

impl fmt::Debug for FuncObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FuncObject::CodeObject { code, const_table } => f
                .debug_struct("CodeObject")
                .field("code", &code)
                .field("const_table", &const_table)
                .finish(),
            FuncObject::NativeFunc { .. } => f.debug_struct("NativeFunc").finish(),
        }
    }
}

impl PartialEq for FuncObject {
    fn eq(&self, other: &FuncObject) -> bool {
        match (self, other) {
            (
                FuncObject::CodeObject {
                    code: code1,
                    const_table: const_table1,
                },
                FuncObject::CodeObject {
                    code: code2,
                    const_table: const_table2,
                },
            ) => (code1 == code2) && (const_table1 == const_table2),
            _ => false,
        }
    }
}