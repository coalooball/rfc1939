use crate::types::response::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_until},
    combinator::{map, opt},
    sequence::terminated,
    sequence::{preceded, tuple},
    IResult,
};
use std::str::from_utf8;

#[derive(Debug, PartialEq)]
pub enum StatusIndicator {
    OK,
    ERR,
}

pub(crate) fn take_until_crlf_consume_crlf(s: &[u8]) -> IResult<&[u8], &[u8]> {
    terminated(take_until("\r\n"), tag(b"\r\n"))(s)
}

pub(crate) fn take_until_crlf(s: &[u8]) -> IResult<&[u8], &[u8]> {
    take_until("\r\n")(s)
}

pub(crate) fn take_until_sp(s: &[u8]) -> IResult<&[u8], &[u8]> {
    take_until(" ")(s)
}

/// A parser parses one line response which only have two parts
/// in which status indicator and messages exist.
pub(crate) fn one_line_response_two_parts_parser<'a, T: OneLine<'a> + Default>(
    s: &'a [u8],
) -> IResult<&[u8], T> {
    map(
        tuple((
            alt((
                map(tag_no_case(b"+OK"), |_| StatusIndicator::OK),
                map(tag_no_case(b"-ERR"), |_| StatusIndicator::ERR),
            )),
            opt(preceded(tag(b" "), take_until_crlf_consume_crlf)),
        )),
        |(si, message)| {
            let mut response = T::default();
            response.set_status_indicator(si);
            response.set_message(if let Some(msg) = message { msg } else { &[] });
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

pub(crate) fn retr_message_parser<'a, T: HaveMessageBody<'a>>(s: &'a [u8]) -> IResult<&[u8], T> {
    map(
        tuple((
            alt((
                map(tag_no_case(b"+OK"), |_| StatusIndicator::OK),
                map(tag_no_case(b"-ERR"), |_| StatusIndicator::ERR),
            )),
            tag(b" "),
            take_until_crlf_consume_crlf,
            opt(terminated(take_until("\r\n.\r\n"), tag(b"\r\n.\r\n"))),
        )),
        |(si, _, msg, body)| {
            let mut tmp_message = T::default();
            tmp_message.set_status_indicator(si);
            tmp_message.set_message(msg);
            tmp_message.set_message_body(body);
            tmp_message
        },
    )(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_untill_crlf() {
        assert_eq!(take_until_crlf(b"1234567\r\n").unwrap().1, b"1234567");
        assert_eq!(take_until_crlf(b"\r\n").unwrap().1, b"");
    }

    #[test]
    fn test_one_line_response_two_parts_parser() {
        assert_eq!(
            one_line_response_two_parts_parser::<Greeting>(b"+OK POP3 server ready\r\n")
                .unwrap()
                .1,
            Greeting {
                status_indicator: StatusIndicator::OK,
                message: b"POP3 server ready"
            }
        )
    }
}
