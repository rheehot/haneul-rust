use crate::constant::Constant;
use crate::error::HaneulError;
use crate::funcobject::FuncObject;
use crate::instruction::Instruction;
use crate::opcode::{BinaryOp, Opcode, UnaryOp};

pub struct StackFrame {
  pub code: Vec<Instruction>,
  pub const_table: Vec<Constant>,
  pub free_vars: Vec<Constant>,
  pub slot_start: usize,
}

#[derive(Default)]
pub struct Machine {
  operand_stack: Vec<Constant>,
  global_vars: Vec<Option<Constant>>,
  global_var_names: Vec<String>,
}

impl Machine {
  pub fn new(global_vars: Vec<Option<Constant>>, global_var_names: Vec<String>) -> Machine {
    let mut vars = global_vars;
    vars.resize(global_var_names.len(), None);

    Machine {
      operand_stack: Vec::new(),
      global_vars: vars,
      global_var_names,
    }
  }

  fn pop(&mut self) -> Option<Constant> {
    self.operand_stack.pop()
  }

  pub fn run(&mut self, frame: &StackFrame) -> Result<(), (u32, HaneulError)> {
    let mut ip = 0;
    let code_length = frame.code.len();

    let result = 'outer: loop {
      if ip >= code_length {
        break Ok(());
      }

      let current_inst = &frame.code[ip];
      // println!("{:?}", self.operand_stack);
      // println!("-----------------");

      match &current_inst.opcode {
        Opcode::Push(v) => {
          self
            .operand_stack
            .push(frame.const_table[*v as usize].clone());
        }
        Opcode::Pop => {
          self.pop();
        }
        Opcode::Load(v) => {
          self
            .operand_stack
            .push(self.operand_stack[frame.slot_start + *v as usize].clone());
        }
        Opcode::LoadDeref(v) => {
          self
            .operand_stack
            .push(frame.free_vars[*v as usize].clone());
        }
        Opcode::LoadGlobal(v) => {
          if let Some(value) = &self.global_vars[*v as usize] {
            self.operand_stack.push(value.clone());
          } else {
            break Err(HaneulError::UnboundVariable {
              var_name: self.global_var_names[*v as usize].clone(),
            });
          }
        }
        Opcode::StoreGlobal(v) => {
          self.global_vars[*v as usize] = self.pop();
        }
        Opcode::Call(given_josa_list) => {
          let given_arity = given_josa_list.len() as u8;
          let value = self.pop().unwrap();

          if let Constant::Function {
            mut josa_map,
            func_object,
          } = value
          {
            let full_arity = josa_map.len();
            let actual_arity = josa_map.values().filter(|x| **x == None).count() as u8;

            if given_arity > actual_arity {
              break Err(HaneulError::TooManyArgs {
                actual_arity,
                given_arity,
              });
            }

            for josa in given_josa_list {
              if josa == "_" {
                for value in josa_map.values_mut() {
                  if *value == None {
                    *value = self.pop();
                    break;
                  }
                }

                continue;
              }

              match josa_map.get_mut(josa) {
                Some(value) => match value {
                  Some(_) => {
                    break 'outer Err(HaneulError::AlreadyAppliedJosa { josa: josa.clone() })
                  }
                  None => {
                    *value = self.pop();
                  }
                },
                None => break 'outer Err(HaneulError::UnboundJosa { josa: josa.clone() }),
              }
            }

            if given_arity < actual_arity {
              self.operand_stack.push(Constant::Function {
                josa_map,
                func_object,
              });
              continue;
            }

            let mut args: Vec<Constant> = josa_map.into_iter().map(|(_, x)| x.unwrap()).collect();

            match func_object {
              FuncObject::CodeObject {
                code,
                const_table,
                free_vars,
              } => {
                let func_frame = StackFrame {
                  code,
                  const_table,
                  free_vars,
                  slot_start: self.operand_stack.len(),
                };

                self.operand_stack.append(&mut args);
                self.run(&func_frame)?;

                let result = self.operand_stack.pop().unwrap();

                for _ in 0..full_arity {
                  self.pop();
                }

                self.operand_stack.push(result)
              }
              FuncObject::NativeFunc { function } => {
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
          let top = self.pop().unwrap();
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
        Opcode::FreeVarLocal(index) => {
          let value = self.operand_stack[frame.slot_start + *index as usize].clone();

          let top = self.operand_stack.last_mut().unwrap();
          if let Constant::Function {
            func_object: FuncObject::CodeObject { free_vars, .. },
            ..
          } = top
          {
            free_vars.push(value);
          } else {
            panic!("FreeVarLocal은 스택의 최상위가 코드 객체인 경우에만 사용 가능합니다.");
          }
        }
        Opcode::FreeVarFree(index) => {
          let value = frame.free_vars[*index as usize].clone();

          let top = self.operand_stack.last_mut().unwrap();
          if let Constant::Function {
            func_object: FuncObject::CodeObject { free_vars, .. },
            ..
          } = top
          {
            free_vars.push(value);
          } else {
            panic!("FreeVarFree는 스택의 최상위가 코드 객체인 경우에만 사용 가능합니다.");
          }
        }
        Opcode::UnaryOp(op) => {
          let value = self.pop().unwrap();
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
          let rhs = self.pop().unwrap();
          let lhs = self.pop().unwrap();

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
