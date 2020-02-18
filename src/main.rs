mod builtin;
mod constant;
mod error;
mod funcobject;
mod instruction;
mod machine;
mod opcode;
mod parser;
mod program;

use std::env;
use std::fs;

use builtin::get_builtin;
use machine::{Machine, StackFrame};
use parser::program;
use program::Program;

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        println!("파일 이름을 입력해주세요.");
        std::process::exit(1);
    }

    let filename = &args.nth(1).unwrap();
    if let Ok(data) = fs::read(filename) {
        let result = program(&data[..]);

        match result {
            Ok((_, Program { const_table, code })) => {
                let mut machine = Machine::new(get_builtin());
                let frame = StackFrame {
                    code,
                    const_table,
                    slot_start: 0,
                };

                match machine.run(&frame) {
                    Ok(_) => println!("정상 종료"),
                    Err((line_number, err)) => {
                        println!("{}번째 라인 에서 에러 발생 : {}", line_number, err)
                    }
                }
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    } else {
        println!("파일을 찾을 수 없습니다.");
    }
}
