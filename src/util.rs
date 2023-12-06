#![allow(dead_code)]

pub fn parse_number(bytes: &[u8]) -> i64 {
    bytes
        .iter()
        .map(|b| *b - b'0')
        .fold(0i64, |sum, digit| sum * 10 + (digit as i64))
}

pub fn parse_number_from_iter(bytes: impl Iterator<Item = u8>) -> i64 {
    bytes
        .map(|b| b - b'0')
        .fold(0i64, |sum, digit| sum * 10 + (digit as i64))
}
