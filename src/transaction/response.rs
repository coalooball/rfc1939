use crate::common::{parse_u8_slice_to_usize_or_0, StatusIndicator};
use crate::types::response::Stat;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map,
    sequence::tuple, IResult,
};

pub(crate) fn stat_parser(s: &[u8]) -> IResult<&[u8], Stat> {
    map(
        tuple((
            alt((
                map(tag(b"+OK"), |_| StatusIndicator::OK),
                map(tag(b"-ERR"), |_| StatusIndicator::ERR),
            )),
            tag(b" "),
            digit1,
            tag(b" "),
            digit1,
        )),
        |(si, _, num, _, size): (StatusIndicator, _, &[u8], _, &[u8])| Stat {
            status_indicator: si,
            number_of_messages: parse_u8_slice_to_usize_or_0(num),
            size_in_octets: parse_u8_slice_to_usize_or_0(size),
        },
    )(s)
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
        stat_parser(b"+OK 2 320").unwrap().1,
        Stat {
            status_indicator: StatusIndicator::OK,
            number_of_messages: 2,
            size_in_octets: 320
        }
    )
}

#[test]
fn test_stat() {
    assert_eq!(
        stat(b"+OK 2 320").unwrap(),
        Stat {
            status_indicator: StatusIndicator::OK,
            number_of_messages: 2,
            size_in_octets: 320
        }
    )
}
