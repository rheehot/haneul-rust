use crate::constant::Constant;
use crate::instruction::Instruction;

#[derive(Debug, PartialEq)]
pub struct Program {
  pub const_table: Vec<Constant>,
  pub code: Vec<Instruction>,
  pub global_var_names: Vec<String>,
}
