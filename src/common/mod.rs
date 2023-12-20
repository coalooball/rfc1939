use crate::types::response::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until1},
    combinator::map,
    sequence::terminated,
    sequence::tuple,
    IResult,
};
use std::str::from_utf8;

#[derive(Debug, PartialEq)]
pub enum StatusIndicator {
    OK,
    ERR,
}

pub(crate) fn take_until_crlf(s: &[u8]) -> IResult<&[u8], &[u8]> {
    terminated(take_until1("\r\n"), tag(b"\r\n"))(s)
}

/// A parser parses one line response which only have two parts
/// in which status indicator and messages exist.
pub(crate) fn one_line_response_two_parts_parser<T: OneLine + Default>(
    s: &[u8],
) -> IResult<&[u8], T> {
    map(
        tuple((
            alt((
                map(tag(b"+OK"), |_| StatusIndicator::OK),
                map(tag(b"-ERR"), |_| StatusIndicator::ERR),
            )),
            tag(b" "),
            take_until_crlf,
        )),
        |(si, _, message)| {
            let mut response = T::default();
            response.set_status_indicator(si);
            response.set_message(message.to_vec());
            response
        },
    )(s)
}

pub(crate) fn parse_u8_slice_to_usize_or_0(s: &[u8]) -> usize {
    if let Ok(str) = from_utf8(s) {
        if let Ok(digit) = str::parse::<usize>(str) {
            digit
        } else {
            0
        }
    } else {
        0
    }
}

#[test]
fn test_take_untill_crlf() {
    assert_eq!(take_until_crlf(b"1234567\r\n").unwrap().1, b"1234567");
}

#[test]
fn test_one_line_response_two_parts_parser() {
    assert_eq!(
        one_line_response_two_parts_parser::<Greeting>(b"+OK POP3 server ready\r\n")
            .unwrap()
            .1,
        Greeting {
            status_indicator: StatusIndicator::OK,
            message: b"POP3 server ready".to_vec()
        }
    )
}
