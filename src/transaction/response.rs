use crate::common::{
    one_line_response_two_parts_parser, parse_u8_slice_to_usize_or_0, StatusIndicator,
};
use crate::types::response::{OneLineTwoParts, Stat};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map,
    sequence::tuple, IResult,
};

pub(crate) fn stat_parser(s: &[u8]) -> IResult<&[u8], Stat> {
    alt((
        map(
            tuple((
                map(tag(b"+OK"), |_| StatusIndicator::OK),
                tag(b" "),
                digit1,
                tag(b" "),
                digit1,
                tag(b"\r\n"),
            )),
            |(si, _, num, _, size, _): (StatusIndicator, _, &[u8], _, &[u8], _)| Stat {
                status_indicator: si,
                number_of_messages: parse_u8_slice_to_usize_or_0(num),
                size_in_octets: parse_u8_slice_to_usize_or_0(size),
                message: vec![],
            },
        ),
        map(one_line_response_two_parts_parser::<OneLineTwoParts>, |x| {
            Stat {
                status_indicator: x.left,
                number_of_messages: 0,
                size_in_octets: 0,
                message: x.right,
            }
        }),
    ))(s)
}

pub fn stat(s: &[u8]) -> Option<Stat> {
    match stat_parser(s) {
        Ok((_, x)) => Some(x),
        Err(_) => None,
    }
}

#[test]
fn test_stat_parser() {
    assert_eq!(
        stat_parser(b"+OK 2 320\r\n").unwrap().1,
        Stat {
            status_indicator: StatusIndicator::OK,
            number_of_messages: 2,
            size_in_octets: 320,
            message: vec![]
        }
    )
}

#[test]
fn test_stat() {
    assert_eq!(
        stat(b"+OK 2 320\r\n").unwrap(),
        Stat {
            status_indicator: StatusIndicator::OK,
            number_of_messages: 2,
            size_in_octets: 320,
            message: vec![]
        }
    );
    assert_eq!(
        stat(b"-ERR failed\r\n").unwrap(),
        Stat {
            status_indicator: StatusIndicator::ERR,
            number_of_messages: 0,
            size_in_octets: 0,
            message: b"failed".to_vec()
        }
    )
}
