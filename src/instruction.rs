use crate::opcode::Opcode;

pub struct Instruction {
  pub line_number: u32,
  pub opcode: Opcode,
}