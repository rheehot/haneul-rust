extern crate nom;

use std::char;
use nom::{IResult, number};

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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn parse_integer() {
        assert_eq!(integer(b"\x00\x00\x00\x00\x00\x00\x00\x0a"), Ok((&b""[..], 10)));
        assert_eq!(integer(b"\x00\x00\x00\x00\x00\x00\x00\x00"), Ok((&b""[..], 0)));
        assert_eq!(integer(b"\xff\xff\xff\xff\xff\xff\xff\xe0"), Ok((&b""[..], -32)));
    }

    #[test]
    fn parse_real() {
        assert_eq!(real(b"\x3f\xf0\x00\x00\x00\x00\x00\x00"), Ok((&b""[..], 1.0)));
        assert_eq!(real(b"\x40\x25\x00\x00\x00\x00\x00\x00"), Ok((&b""[..], 10.5)));
        assert_eq!(real(b"\xc0\x59\x00\x00\x00\x00\x00\x00"), Ok((&b""[..], -100.0)));
        assert_eq!(real(b"\x00\x00\x00\x00\x00\x00\x00\x00"), Ok((&b""[..], 0.0)));
    }

    #[test]
    fn parse_char() {
        assert_eq!(char(b"\x00\x00\x00\x61"), Ok((&b""[..], Some('a'))));
        assert_eq!(char(b"\x00\x00\xac\x00"), Ok((&b""[..], Some('가'))));
        assert_eq!(char(b"\x00\x01\xf6\x3b"), Ok((&b""[..], Some('😻'))));
        assert_eq!(char(b"\x00\x02\x10\x7b"), Ok((&b""[..], Some('𡁻'))));
    }
}