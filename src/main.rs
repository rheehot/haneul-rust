mod opcode;
mod instruction;

use opcode::Opcode;
use instruction::Instruction;

fn main() {
  let inst = Instruction {
    line_number: 1,
    opcode: Opcode::Push(10),
  };

  println!("{:#?}", inst);
}
