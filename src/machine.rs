use std::collections::HashMap;

use crate::constant::Constant;
use crate::error::HaneulError;
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
  pub fn new(global_vars: HashMap<String, Constant>) -> Machine {
    Machine {
      operand_stack: Vec::new(),
      global_vars,
    }
  }

  pub fn run(&mut self, frame: &StackFrame) -> Result<(), (u32, HaneulError)> {
    let mut ip = 0;
    let code_length = frame.code.len();

    let result = loop {
      if ip >= code_length {
        break Ok(());
      }

      let current_inst = &frame.code[ip];
      // println!("{:?}", current_inst);

      match &current_inst.opcode {
        Opcode::Push(v) => {
          self
            .operand_stack
            .push(frame.const_table[*v as usize].clone());
        }
        Opcode::Pop => {
          self.operand_stack.pop();
        }
        Opcode::Load(v) => {
          self
            .operand_stack
            .push(self.operand_stack[frame.slot_start + *v as usize].clone());
        }
        Opcode::LoadGlobal(v) => {
          if let Some(value) = self.global_vars.get(v) {
            self.operand_stack.push(value.clone());
          } else {
            break Err(HaneulError::UnboundVariable {
              var_name: v.clone(),
            });
          }
        }
        Opcode::StoreGlobal(v) => {
          self
            .global_vars
            .insert(v.clone(), self.operand_stack.pop().unwrap());
        }
        Opcode::Call(given_arity) => {
          let value = self.operand_stack.pop().unwrap();

          if let Constant::Function {
            arity: actual_arity,
            func_object,
          } = value
          {
            if *given_arity != actual_arity {
              break Err(HaneulError::TooManyArgs {
                actual_arity,
                given_arity: *given_arity,
              });
            }

            match func_object {
              FuncObject::CodeObject { code, const_table } => {
                let func_frame = StackFrame {
                  code,
                  const_table,
                  slot_start: self.operand_stack.len() - *given_arity as usize,
                };

                self.run(&func_frame)?;
                let result = self.operand_stack.pop().unwrap();

                // self.operand_stack
                //     .resize(func_frame.slot_start, Constant::None);
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
            break Err(HaneulError::NotCallable { value });
          }
        }
        Opcode::Jmp(v) => {
          ip = *v as usize;
          continue;
        }
        Opcode::PopJmpIfFalse(v) => {
          let top = self.operand_stack.pop().unwrap();
          match top {
            Constant::Boolean(value) => {
              if !value {
                ip = *v as usize;
                continue;
              }
            }
            _ => break Err(HaneulError::ExpectedBoolean { value: top }),
          };
        }
        Opcode::UnaryOp(op) => {
          let value = self.operand_stack.pop().unwrap();
          let result = match op {
            UnaryOp::Negate => -&value,
          };

          match result {
            Some(result_value) => self.operand_stack.push(result_value),
            None => {
              break Err(HaneulError::InvalidUnaryOp {
                value,
                op: op.clone(),
              })
            }
          }
        }
        Opcode::BinaryOp(op) => {
          let rhs = self.operand_stack.pop().unwrap();
          let lhs = self.operand_stack.pop().unwrap();

          let result = match op {
            BinaryOp::Add => &lhs + &rhs,
            BinaryOp::Subtract => &lhs - &rhs,
            BinaryOp::Multiply => &lhs * &rhs,
            BinaryOp::Divide => &lhs / &rhs,
            BinaryOp::Mod => &lhs % &rhs,
            BinaryOp::Cmp(ord) => {
              PartialOrd::partial_cmp(&lhs, &rhs).map(|v| Constant::Boolean(v == *ord))
            }
          };

          match result {
            Some(result_value) => self.operand_stack.push(result_value),
            None => {
              break Err(HaneulError::InvalidBinaryOp {
                lhs,
                rhs,
                op: op.clone(),
              })
            }
          }
        }
      }

      ip += 1;
    };

    match result {
      Ok(_) => Ok(()),
      Err(err) => Err((frame.code[ip].line_number, err)),
    }
  }
}
