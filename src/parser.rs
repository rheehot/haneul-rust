extern crate nom;

use nom::{combinator, multi, number::complete::*, IResult};
use std::char;
use std::cmp::Ordering;

use crate::constant::Constant;
use crate::funcobject::FuncObject;
use crate::instruction::Instruction;
use crate::opcode::{BinaryOp, Opcode, UnaryOp};
use crate::program::Program;

fn integer(input: &[u8]) -> IResult<&[u8], i64> {
    be_i64(input)
}

fn real(input: &[u8]) -> IResult<&[u8], f64> {
    be_f64(input)
}

fn character(input: &[u8]) -> IResult<&[u8], char> {
    let (input, result) = be_u32(input)?;
    Ok((input, char::from_u32(result).unwrap()))
}

fn boolean(input: &[u8]) -> IResult<&[u8], bool> {
    let (input, result) = be_u8(input)?;
    Ok((input, result == 1))
}

fn list<A, F>(input: &[u8], parser: F) -> IResult<&[u8], Vec<A>>
where
    F: Fn(&[u8]) -> IResult<&[u8], A>,
{
    let (input, count) = be_u64(input)?;
    multi::count(parser, count as usize)(input)
}

fn char_utf8(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    let (input, head) = combinator::peek(be_u8)(input)?;
    let count = match () {
        _ if head < 0x80 => 1,
        _ if head < 0xE0 => 2,
        _ if head < 0xF0 => 3,
        _ => 4,
    };
    multi::count(be_u8, count)(input)
}

fn string(input: &[u8]) -> IResult<&[u8], String> {
    // let (input, count) = integer(input)?;
    // let (input, result) = multi::count(char_utf8, count as usize)(input)?;
    let (input, result) = list(input, char_utf8)?;
    let flattened = result.into_iter().flatten().collect();
    Ok((input, String::from_utf8(flattened).unwrap()))
}

fn apply<A, B, C>(value: (A, B), f: fn(B) -> C) -> (A, C) {
    let (a, b) = value;
    (a, f(b))
}

fn instruction(input: &[u8]) -> IResult<&[u8], Instruction> {
    let (input, line_number) = be_u32(input)?;
    let (input, opcode_index) = be_u8(input)?;
    let (input, opcode) = match opcode_index {
        0 => apply(be_u32(input)?, Opcode::Push),
        1 => (input, Opcode::Pop),
        2 => apply(be_u32(input)?, Opcode::Load),
        3 => apply(string(input)?, Opcode::StoreGlobal),
        4 => apply(string(input)?, Opcode::LoadGlobal),
        5 => apply(be_u32(input)?, Opcode::Call),
        6 => apply(be_u32(input)?, Opcode::Jmp),
        7 => apply(be_u32(input)?, Opcode::PopJmpIfFalse),
        8 => (input, Opcode::BinaryOp(BinaryOp::Add)),
        9 => (input, Opcode::BinaryOp(BinaryOp::Subtract)),
        10 => (input, Opcode::BinaryOp(BinaryOp::Multiply)),
        11 => (input, Opcode::BinaryOp(BinaryOp::Divide)),
        12 => (input, Opcode::BinaryOp(BinaryOp::Mod)),
        13 => (input, Opcode::BinaryOp(BinaryOp::Cmp(Ordering::Equal))),
        14 => (input, Opcode::BinaryOp(BinaryOp::Cmp(Ordering::Less))),
        15 => (input, Opcode::BinaryOp(BinaryOp::Cmp(Ordering::Greater))),
        16 => (input, Opcode::UnaryOp(UnaryOp::Negate)),
        _ => panic!("invalid opcode type value"),
    };

    Ok((
        input,
        Instruction {
            line_number,
            opcode,
        },
    ))
}

fn code_object(input: &[u8]) -> IResult<&[u8], FuncObject> {
    let (input, const_table) = list(input, constant)?;
    let (input, code) = list(input, instruction)?;

    Ok((input, FuncObject::CodeObject { const_table, code }))
}

fn constant(input: &[u8]) -> IResult<&[u8], Constant> {
    let (input, constant_index) = be_u8(input)?;
    let (input, constant) = match constant_index {
        0 => (input, Constant::None),
        1 => {
            let (input, value) = integer(input)?;
            (input, Constant::Integer(value))
        }
        2 => {
            let (input, value) = real(input)?;
            (input, Constant::Real(value))
        }
        3 => {
            let (input, value) = character(input)?;
            (input, Constant::Char(value))
        }
        4 => {
            let (input, value) = boolean(input)?;
            (input, Constant::Boolean(value))
        }
        5 => {
            let (input, arity) = be_u8(input)?;
            let (input, value) = code_object(input)?;
            (
                input,
                Constant::Function {
                    arity,
                    func_object: value,
                },
            )
        }
        _ => panic!("invalid constant type value"),
    };
    Ok((input, constant))
}

pub fn program(input: &[u8]) -> IResult<&[u8], Program> {
    let (input, const_table) = list(input, constant)?;
    let (input, code) = list(input, instruction)?;

    Ok((input, Program { const_table, code }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_integer() {
        assert_eq!(
            integer(b"\x00\x00\x00\x00\x00\x00\x00\x0a"),
            Ok((&b""[..], 10))
        );
        assert_eq!(
            integer(b"\x00\x00\x00\x00\x00\x00\x00\x00"),
            Ok((&b""[..], 0))
        );
        assert_eq!(
            integer(b"\xff\xff\xff\xff\xff\xff\xff\xe0"),
            Ok((&b""[..], -32))
        );
    }

    #[test]
    fn parse_real() {
        assert_eq!(
            real(b"\x3f\xf0\x00\x00\x00\x00\x00\x00"),
            Ok((&b""[..], 1.0))
        );
        assert_eq!(
            real(b"\x40\x25\x00\x00\x00\x00\x00\x00"),
            Ok((&b""[..], 10.5))
        );
        assert_eq!(
            real(b"\xc0\x59\x00\x00\x00\x00\x00\x00"),
            Ok((&b""[..], -100.0))
        );
        assert_eq!(
            real(b"\x00\x00\x00\x00\x00\x00\x00\x00"),
            Ok((&b""[..], 0.0))
        );
    }

    #[test]
    fn parse_char() {
        assert_eq!(character(b"\x00\x00\x00\x61"), Ok((&b""[..], 'a')));
        assert_eq!(character(b"\x00\x00\xac\x00"), Ok((&b""[..], '가')));
        assert_eq!(character(b"\x00\x01\xf6\x3b"), Ok((&b""[..], '😻')));
        assert_eq!(character(b"\x00\x02\x10\x7b"), Ok((&b""[..], '𡁻')));
    }

    #[test]
    fn parse_boolean() {
        assert_eq!(boolean(b"\x00"), Ok((&b""[..], false)));
        assert_eq!(boolean(b"\x01"), Ok((&b""[..], true)));
    }

    #[test]
    fn parse_string() {
        assert_eq!(
            string(b"\x00\x00\x00\x00\x00\x00\x00\x03\xea\xb0\x80\xeb\x82\x98\xeb\x8b\xa4"),
            Ok((&b""[..], String::from("가나다")))
        );
        assert_eq!(
            string(b"\x00\x00\x00\x00\x00\x00\x00\x0a\xec\x95\x88\xeb\x85\x95\x20\x61\x62\x63\x20\x31\x32\x33"),
            Ok((&b""[..], String::from("안녕 abc 123")))
        );
        assert_eq!(
            string(b"\x00\x00\x00\x00\x00\x00\x00\x0e\xec\x95\x88\xeb\x85\x95\x20\xf0\x9f\x98\xaf\x20\xf0\x9f\x98\xaa\x20\xf0\x9f\x98\xab\x20\xf0\x9f\x98\xb4\x20\xf0\x9f\x98\x8c\x20\xf0\x9f\x98\x9b"),
            Ok((&b""[..], String::from("안녕 😯 😪 😫 😴 😌 😛")))
        );
    }

    #[test]
    fn parse_instruction() {
        assert_eq!(
            instruction(b"\x00\x00\x00\x0a\x01"),
            Ok((
                &b""[..],
                Instruction {
                    line_number: 10,
                    opcode: Opcode::Pop
                }
            ))
        );
        assert_eq!(
            instruction(
                b"\x00\x00\x01\xa7\x04\x00\x00\x00\x00\x00\x00\x00\x02\xec\x82\xac\xea\xb3\xbc"
            ),
            Ok((
                &b""[..],
                Instruction {
                    line_number: 423,
                    opcode: Opcode::LoadGlobal(String::from("사과"))
                }
            ))
        );
    }

    #[test]
    fn parse_constant() {
        assert_eq!(constant(b"\x00"), Ok((&b""[..], Constant::None)));
        assert_eq!(
            constant(b"\x01\x00\x00\x00\x00\x00\x00\x00\x7b"),
            Ok((&b""[..], Constant::Integer(123)))
        );
        assert_eq!(
            constant(b"\x02\x40\x25\x00\x00\x00\x00\x00\x00"),
            Ok((&b""[..], Constant::Real(10.5)))
        );
        assert_eq!(
            constant(b"\x03\x00\x00\xc5\x48"),
            Ok((&b""[..], Constant::Char('안')))
        );
        assert_eq!(
            constant(b"\x04\x01"),
            Ok((&b""[..], Constant::Boolean(true)))
        );

        let code_object = FuncObject::CodeObject {
            code: vec![
                Instruction {
                    line_number: 1,
                    opcode: Opcode::Load(0),
                },
                Instruction {
                    line_number: 1,
                    opcode: Opcode::Push(0),
                },
                Instruction {
                    line_number: 1,
                    opcode: Opcode::BinaryOp(BinaryOp::Add),
                },
            ],
            const_table: vec![Constant::Integer(1)],
        };

        assert_eq!(
            constant(b"\x05\x01\x00\x00\x00\x00\x00\x00\x00\x01\x01\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x03\x00\x00\x00\x01\x02\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x01\x08"),
            Ok((&b""[..], Constant::Function {
              arity: 1,
              func_object: code_object
            })
        ));
    }
}
