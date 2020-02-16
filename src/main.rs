mod instruction;
mod opcode;
mod parser;

use instruction::Instruction;
use opcode::Opcode;

fn main() {
    let inst = Instruction {
        line_number: 1,
        opcode: Opcode::Push(10),
    };

    println!("{:#?}", inst);
}
