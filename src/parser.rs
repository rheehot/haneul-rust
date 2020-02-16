extern crate nom;

use nom::{character, combinator, multi, number, IResult};
use std::char;

fn integer(input: &[u8]) -> IResult<&[u8], i64> {
    number::complete::be_i64(input)
}

fn real(input: &[u8]) -> IResult<&[u8], f64> {
    number::complete::be_f64(input)
}

fn char(input: &[u8]) -> IResult<&[u8], Option<char>> {
    let (input, result) = number::complete::be_u32(input)?;
    Ok((input, char::from_u32(result)))
}

fn boolean(input: &[u8]) -> IResult<&[u8], bool> {
    let (input, result) = number::complete::be_u8(input)?;
    Ok((input, result == 1))
}

fn char_utf8(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    let (input, head) = combinator::peek(number::complete::be_u8)(input)?;
    let count = match () {
        _ if head < 0x80 => 1,
        _ if head < 0xE0 => 2,
        _ if head < 0xF0 => 3,
        _ => 4,
    };
    multi::count(number::complete::be_u8, count)(input)
}

fn string(input: &[u8]) -> IResult<&[u8], String> {
    let (input, count) = integer(input)?;
    let (input, result) = multi::count(char_utf8, count as usize)(input)?;
    let flatten = result.into_iter().flatten().collect();
    Ok((input, String::from_utf8(flatten).unwrap()))
}

// fn instruction(input: &[u8]) -> IResult<&[u8], bool> {
//   let (input, result) = number::complete::be_u8(input)?;
// }

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
}
