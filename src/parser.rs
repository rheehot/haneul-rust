extern crate nom;

use nom::{character, combinator, multi, number::complete::*, IResult};
use std::char;

use crate::instruction::Instruction;
use crate::opcode::Opcode;

fn integer(input: &[u8]) -> IResult<&[u8], i64> {
    be_i64(input)
}

fn real(input: &[u8]) -> IResult<&[u8], f64> {
    be_f64(input)
}

fn char(input: &[u8]) -> IResult<&[u8], Option<char>> {
    let (input, result) = be_u32(input)?;
    Ok((input, char::from_u32(result)))
}

fn boolean(input: &[u8]) -> IResult<&[u8], bool> {
    let (input, result) = be_u8(input)?;
    Ok((input, result == 1))
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
    let (input, count) = integer(input)?;
    let (input, result) = multi::count(char_utf8, count as usize)(input)?;
    let flatten = result.into_iter().flatten().collect();
    Ok((input, String::from_utf8(flatten).unwrap()))
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
        8 => (input, Opcode::Add),
        9 => (input, Opcode::Subtract),
        10 => (input, Opcode::Multiply),
        11 => (input, Opcode::Divide),
        12 => (input, Opcode::Mod),
        13 => (input, Opcode::Equal),
        14 => (input, Opcode::LessThan),
        15 => (input, Opcode::GreaterThan),
        16 => (input, Opcode::Negate),
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
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
        assert_eq!(char(b"\x00\x00\x00\x61"), Ok((&b""[..], Some('a'))));
        assert_eq!(char(b"\x00\x00\xac\x00"), Ok((&b""[..], Some('가'))));
        assert_eq!(char(b"\x00\x01\xf6\x3b"), Ok((&b""[..], Some('😻'))));
        assert_eq!(char(b"\x00\x02\x10\x7b"), Ok((&b""[..], Some('𡁻'))));
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
}
