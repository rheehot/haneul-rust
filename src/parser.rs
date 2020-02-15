extern crate nom;

use nom::{IResult, number};

fn integer(input: &[u8]) -> IResult<&[u8], i64> {
  number::complete::be_i64(input)
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
}