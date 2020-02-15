use crate::opcode::Opcode;

#[derive(Debug)]
pub struct Instruction {
  pub line_number: u32,
  pub opcode: Opcode,
}