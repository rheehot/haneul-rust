use std::collections::HashMap;

use crate::constant::Constant;
use crate::funcobject::FuncObject;
use crate::instruction::Instruction;
use crate::opcode::{BinaryOp, Opcode, UnaryOp};

pub struct StackFrame {
    pub code: Vec<Instruction>,
    pub const_table: Vec<Constant>,
    pub slot_start: usize,
}

#[derive(Default)]
pub struct Machine {
    operand_stack: Vec<Constant>,
    global_vars: HashMap<String, Constant>,
}

impl Machine {
    pub fn run(&mut self, frame: &StackFrame) {
        let mut ip = 0;
        let code_length = frame.code.len();

        while ip < code_length {
            let current_inst = &frame.code[ip];

            match &current_inst.opcode {
                Opcode::Push(v) => {
                    self.operand_stack
                        .push(frame.const_table[*v as usize].clone());
                }
                Opcode::Pop => {
                    self.operand_stack.pop();
                }
                Opcode::Load(v) => {
                    self.operand_stack
                        .push(self.operand_stack[frame.slot_start + *v as usize].clone());
                }
                Opcode::LoadGlobal(v) => {
                    if let Some(value) = self.global_vars.get(v) {
                        self.operand_stack.push(value.clone());
                    } else {
                        panic!(format!("변수 {}을(를) 찾을 수 없습니다.", v))
                    }
                }
                Opcode::StoreGlobal(v) => {
                    self.global_vars
                        .insert(v.clone(), self.operand_stack.pop().unwrap());
                }
                Opcode::Call(given_arity) => {
                    if let Constant::Function {
                        arity: actual_arity,
                        func_object,
                    } = self.operand_stack.pop().unwrap()
                    {
                        if *given_arity as u8 != actual_arity {
                            panic!(format!(
                                "이 함수는 {}개의 인수를 받지만 {}개가 주어졌습니다.",
                                actual_arity, given_arity
                            ))
                        }

                        match func_object {
                            FuncObject::CodeObject { code, const_table } => {
                                let func_frame = StackFrame {
                                    code,
                                    const_table,
                                    slot_start: self.operand_stack.len() - *given_arity as usize,
                                };

                                self.run(&func_frame);
                                let result = self.operand_stack.pop().unwrap();

                                for _ in 0..*given_arity {
                                    self.operand_stack.pop();
                                }

                                self.operand_stack.push(result)
                            }
                            FuncObject::NativeFunc { function } => {
                                let mut args = Vec::new();

                                for _ in 0..*given_arity {
                                    args.push(self.operand_stack.pop().unwrap())
                                }

                                self.operand_stack.push(function(args));
                            }
                        }
                    } else {
                        panic!("이 타입은 호출 가능한 타입이 아닙니다.")
                    }
                }
                Opcode::Jmp(v) => {
                    ip = *v as usize;
                    continue;
                }
                Opcode::PopJmpIfFalse(v) => {
                    match self.operand_stack.pop().unwrap() {
                        Constant::Boolean(value) => {
                            if !value {
                                ip = *v as usize;
                                continue;
                            }
                        }
                        _ => panic!("여기에는 참 또는 거짓 타입을 필요로 합니다."),
                    };
                }
                Opcode::UnaryOp(op) => {
                    let value = self.operand_stack.pop().unwrap();
                    let result = match op {
                        UnaryOp::Negate => -&value,
                    };

                    match result {
                        Some(result_value) => self.operand_stack.push(result_value),
                        None => panic!("이 타입에는 연산을 적용할 수 없습니다."),
                    }
                }
                Opcode::BinaryOp(op) => {
                    let rhs = &self.operand_stack.pop().unwrap();
                    let lhs = &self.operand_stack.pop().unwrap();

                    let result = match op {
                        BinaryOp::Add => lhs + rhs,
                        BinaryOp::Subtract => lhs - rhs,
                        BinaryOp::Multiply => lhs * rhs,
                        BinaryOp::Divide => lhs / rhs,
                        BinaryOp::Mod => lhs % rhs,
                        BinaryOp::Cmp(ord) => Some(Constant::Boolean(
                            PartialOrd::partial_cmp(&lhs, &rhs) == Some(*ord),
                        )),
                    };

                    match result {
                        Some(result_value) => self.operand_stack.push(result_value),
                        None => panic!("이 타입에는 연산을 적용할 수 없습니다."),
                    }
                }
            }

            ip += 1;
        }
    }
}
